use anyhow::Result;

pub mod config;
pub mod connection;
pub mod query;
pub mod tenant;
pub mod error;

use config::Config;
use odbc_api::{
    ConnectionOptions,
    Environment,
    Cursor,
    buffers::TextRowSet
};

use std::sync::Arc;
use std::collections::HashMap;

use tonic::{transport::Server, Request, Response, Status};

use db_proxy::db_proxy_server::{DbProxy, DbProxyServer};
use db_proxy::{QueryRequest, QueryReply};

pub mod db_proxy {
    tonic::include_proto!("db_proxy");
}

#[derive(Debug)]
pub struct MyProxy {
    env: Arc<Environment>,
    connection_strings: Arc<HashMap<String, String>>,
}

impl MyProxy {
    pub fn new(env: Arc<Environment>, connection_strings: Arc<HashMap<String, String>>) -> Self {
        MyProxy { env, connection_strings }
    }
}

#[tonic::async_trait]
impl DbProxy for MyProxy {
    async fn send_query(
        &self,
        request: Request<QueryRequest>,
    ) -> Result<Response<QueryReply>, Status> {
        println!("Got a request: {:?}", request);
        let req = request.into_inner();
        let tenant_id = req.tenant_id;
        let query = req.query;

        // Look up the connection string for the tenant
        let conn_str = self.connection_strings.get(&tenant_id).ok_or_else(|| {
            Status::not_found(format!("No connection string found for tenant ID: {}", tenant_id))
        })?;

        // Create a new connection for this request
        let conn = self.env
        .connect_with_connection_string(conn_str, ConnectionOptions::default())
        .map_err(|e| Status::internal(format!("Failed to connect: {}", e)))?;

        // Execute the query
        let x = match conn.execute(&query, (), None) {
            Ok(Some(mut cursor)) => {
                let mut rows = Vec::new();
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))
                    .map_err(|e| Status::internal(format!("Failed to create buffer: {}", e)))?;
                let mut row_set_cursor = cursor
                    .bind_buffer(&mut buffers)
                    .map_err(|e| Status::internal(format!("Failed to bind buffer: {}", e)))?;

                // Collect rows as strings
                while let Some(batch) = row_set_cursor
                    .fetch()
                    .map_err(|e| Status::internal(format!("Failed to fetch: {}", e)))?
                {
                    for row_index in 0..batch.num_rows() {
                        let row: Vec<String> = (0..batch.num_cols())
                            .map(|col_index| {
                                String::from_utf8_lossy(
                                    batch.at(col_index, row_index).unwrap_or(&[]),
                                )
                                .into_owned()
                            })
                            .collect();
                        rows.push(row.join(","));
                    }
                }

                Ok(Response::new(QueryReply {
                    rows,
                    error: String::new(),
                }))
            }
            Ok(None) => Ok(Response::new(QueryReply {
                rows: vec![],
                error: "Query returned no results".to_string(),
            })),
            Err(e) => Ok(Response::new(QueryReply {
                rows: vec![],
                error: format!("Query failed: {}", e),
            })),
        };
        x
    }
}

/// Maximum number of rows fetched with one row set. Fetching batches of rows is usually much
/// faster than fetching individual rows.
/// grpcurl -plaintext -import-path ./proto -proto db_proxy.proto -d '{"tenant_id": "tenant1", "query": "SELECT * FROM users"}' '[::1]:50051' db_proxy.DbProxy/SendQuery
const BATCH_SIZE: usize = 5000;

#[tokio::main]
async fn main() -> Result<()> {
    unsafe {
        odbc_api::Environment::set_connection_pooling(odbc_api::sys::AttrConnectionPooling::DriverAware)
            .map_err(|e| anyhow::anyhow!("Failed to set connection pooling: {}", e))?;
    }

    let config = Config::load()?;
    let env = Arc::new(Environment::new()?);
    let addr = "[::1]:50051".parse()?;

    // Create hash map of tenant IDs and connection strings
    let connection_strings = Arc::new(
        config.tenants.iter().map(|(k, v)| (k.clone(), v.connection_string.clone())).collect()
    );

    let proxy = MyProxy::new(Arc::clone(&env), Arc::clone(&connection_strings));

    Server::builder()
        .add_service(DbProxyServer::new(proxy))
        .serve(addr)
        .await?;

    Ok(())
}