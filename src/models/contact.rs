use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Contact {
    pub name: String,
    pub phone_no: i64,
    pub email: String,
}
