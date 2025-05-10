use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProxyError {
    #[error("ODBC error: {0}")]
    Odbc(#[from] odbc_api::Error),
    #[error("Configuration error: {0}")]
    Config(#[from] anyhow::Error),
    #[error("Tenant not found: {0}")]
    TenantNotFound(String),
    #[error("Query execution failed: {0}")]
    Query(String),
}