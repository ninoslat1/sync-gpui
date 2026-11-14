use mysql::{Pool, prelude::Queryable};

use crate::{db::models::LocationModel, types::{DbConfig, Location}};

pub struct LocationRepository {
    pool: Pool
}

impl LocationRepository {
    pub fn connect(cfg: &DbConfig) -> Result<Self, String> {
        Ok(Self {pool: cfg.pool()?})
    }

    pub fn from_pool(pool: Pool) -> Self {
        Self{ pool }
    }

    pub fn load_location_list(&self) -> Result<Vec<Location>, String> {
        let mut conn = self.pool.get_conn().map_err(|e| e.to_string())?;
        let rows: Vec<(i32, String)> = conn.query("SELECT id, name FROM `bromomst`.`config` ORDER BY name").map_err(|e| e.to_string())?;

        Ok(rows.into_iter().map(|(id, name)| Location {id, name}).collect())
    }
}
