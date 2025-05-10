// use crate::connection::OdbcConnection;
// use crate::error::ProxyError;
// use anyhow::Result;
// use std::sync::Arc;
// use tokio::sync::Mutex;
// use odbc_api::{ Connection, Environment, ConnectionOptions};

// pub struct ConnectionPool {
//     connections: Arc<Mutex<Vec<Connection<'static>>>>,
// }

// impl ConnectionPool {
//     pub fn new(connection_string: &str, size: usize) -> Result<Self, ProxyError> {
//         let env = Arc::new(Environment::new()?);
//         let mut connections = Vec::with_capacity(size);
//         for _ in 0..size {
//             let conn = env.connect_with_connection_string(connection_string, ConnectionOptions::default())?;
//             connections.push(conn);
//         }
//         Ok(Self{connections: Arc::new(Mutex::new(connections))})
//     }

//     pub async fn get(&self) -> Result<Connection, ProxyError> {
//         let mut conns = self.connections.lock().await;
//         conns.pop().ok_or_else(|| ProxyError::Query("No available connections".into()))
//     }
// }