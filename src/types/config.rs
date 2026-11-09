use gpui_kit::*;
use gpui_kit::component::input::InputState;

pub struct DBConfigForm {
    pub db_host: Entity<InputState>,
    pub db_port: Entity<InputState>,
    pub db_name: Entity<InputState>,
    pub db_username: Entity<InputState>,
    pub db_password: Entity<InputState>,
}

pub struct PosConfigForm {
    pub pos_api_url: Entity<InputState>,
    pub pos_token: Entity<InputState>,
    pub pos_type: Entity<InputState>,
    pub pos_location_code: Entity<InputState>,
    pub pos_area_code: Entity<InputState>,
    pub pos_grace_period: Entity<InputState>,
    pub pos_ping_interval: Entity<InputState>,
}