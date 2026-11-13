use gpui_kit::*;
use gpui_kit::component::input::InputState;
use serde::{Serialize, Deserialize};

pub struct DBConfigForm {
    pub db_host: Entity<InputState>,
    pub db_port: Entity<InputState>,
    // pub db_name: Entity<InputState>,
    pub db_username: Entity<InputState>,
    pub db_password: Entity<InputState>,
}

pub struct PosConfigForm {
    pub pos_api_url: Entity<InputState>,
    pub pos_token: Entity<InputState>,
    pub pos_location_code: Entity<InputState>,
    pub pos_area_code: Entity<InputState>,
    pub pos_grace_period: Entity<InputState>,
    pub pos_ping_interval: Entity<InputState>,
    pub location_id: Option<i32>,
    pub area_id: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct StoredDbConfig {
    pub host: String,
    pub port: u16,
    // pub name: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct StoredPosConfig {
    pub api_url: String,
    pub token: String,
    pub location_code: Option<i32>,
    pub area_code: Option<i32>,
    pub grace_period: Option<i32>,
    pub ping_interval: Option<i32>
}
