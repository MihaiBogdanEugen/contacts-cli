use crate::db::schema::contacts;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = contacts)]
pub struct Contact {
    pub name: String,
    pub phone_no: i64,
    pub email: String,
}
