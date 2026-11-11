use mysql::{prelude::Queryable, OptsBuilder, Pool, PoolConstraints, PoolOpts};
use std::time::Duration;

use crate::types::DbConfig;

impl DbConfig {
    fn opts(&self) -> OptsBuilder {
        OptsBuilder::new()
            .ip_or_hostname(Some(self.host.clone()))
            .tcp_port(self.port)
            .db_name(Some(self.name.clone()))
            .user(Some(self.username.clone()))
            .pass(Some(self.password.clone()))
            .tcp_connect_timeout(Some(Duration::from_secs(5)))
            .pool_opts(
                PoolOpts::default()
                    .with_constraints(PoolConstraints::new(1, 4).unwrap()),
            )
    }

    pub fn pool(&self) -> Result<Pool, String> {
        Pool::new(self.opts()).map_err(|e| e.to_string())
    }
}

/// Returns the server version on success.
pub fn test_connection(cfg: &DbConfig) -> Result<String, String> {
    let pool = cfg.pool()?;
    let mut conn = pool.get_conn().map_err(|e| e.to_string())?;
    let version: Option<String> = conn
        .query_first("SELECT VERSION()")
        .map_err(|e| e.to_string())?;
    Ok(version.unwrap_or_default())
}