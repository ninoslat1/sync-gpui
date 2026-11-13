
use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct LocationModel {
    pub id: i32,

    pub name: String,
    pub address1: String,
    pub address2: String,
    pub loc_code: String,

    pub android_port: i32,
    pub hppc_port: i32,
    pub ig_port: i32,

    pub lot_payment: i8,
    pub lot_closing: i8,

    pub user_id: i32,

    pub customer_id: Option<i32>,
    pub live_date: Option<NaiveDateTime>,
    pub pic_id: Option<i32>,

    pub uid: Option<String>,
    pub code: Option<String>,
    pub token: Option<String>,
    pub token_last_refetched: Option<NaiveDateTime>,

    pub tz: Option<String>,
    pub display_url: Option<String>,

    pub r#type: String,
}
