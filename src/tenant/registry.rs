// use crate::config::Config;
// use crate::connection::ConnectionPool;
// use crate::error::ProxyError;
// use anyhow::Result;
// use std::collections::HashMap;

// pub struct TenantRegistry<'a> {
//     pools: HashMap<String, ConnectionPool<'a>>,
// }

// impl<'a> TenantRegistry<'a> {
//     pub async fn new(config: &Config) -> Result<Self, ProxyError> {
//         let mut pools = HashMap::new();
//         for (tenant_id, tenant_config) in &config.tenants {
//             let pool = ConnectionPool::new(&tenant_config.connection_string, 5)?;
//             pools.insert(tenant_id.clone(), pool);
//         }
//         Ok(TenantRegistry { pools })
//     }

//     // pub async fn get_pool(&self, tenant_id: &str) -> Result<&ConnectionPool, ProxyError> {
//     //     self.pools
//     //         .get(tenant_id)
//     //         .ok_or_else(|| ProxyError::TenantNotFound(tenant_id.to_string()))
//     // }
// }