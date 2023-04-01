use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Contact {
    pub name: String,
    pub phone_no: u64,
    pub email: String,
}
