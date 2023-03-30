use dotenvy::dotenv;
use std::env;

use crate::{models::contact::Contact, repositories::contacts::ContactsRepository};

use super::contacts::{is_valid_email, is_valid_phone_no};

const DATABASE_URL_KEY: &str = "DATABASE_URL";
pub struct DbContactsRepository {}

impl Default for DbContactsRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl DbContactsRepository {
    pub fn new() -> Self {
        DbContactsRepository {}
    }

    
}

impl ContactsRepository for DbContactsRepository {
    fn add(
        &mut self,
        name: String,
        phone_no_as_string: String,
        email: String,
    ) -> Result<(), String> {
        if name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }

        match is_valid_email(&email) {
            Ok(is_valid_email) => {
                if !is_valid_email {
                    return Err("Email is not valid".to_string());
                }
            }
            Err(err) => return Err(err.to_string()),
        }

        match is_valid_phone_no(&phone_no_as_string) {
            Ok(is_valid_phone_no) => {
                if !is_valid_phone_no {
                    return Err("Phone no is not valid".to_string());
                }
            }
            Err(err) => return Err(err.to_string()),
        }

        let phone_no: i64 = match phone_no_as_string.parse::<i64>() {
            Ok(x) => x,
            Err(err) => return Err(err.to_string()),
        };

        todo!()
    }

    fn update_email(&mut self, name: &str, new_email: String) -> Result<bool, String> {
        match is_valid_email(&new_email) {
            Ok(is_valid_email) => {
                if !is_valid_email {
                    return Err("New email is not valid".to_string());
                }
            }
            Err(err) => return Err(err.to_string()),
        }

        todo!()
    }

    fn update_phone_no(
        &mut self,
        name: &str,
        new_phone_no_as_string: String,
    ) -> Result<bool, String> {
        match is_valid_phone_no(&new_phone_no_as_string) {
            Ok(is_valid_phone_no) => {
                if !is_valid_phone_no {
                    return Err("New phone no is not valid".to_string());
                }
            }
            Err(err) => return Err(err.to_string()),
        }

        let new_phone_no: i64 = match new_phone_no_as_string.parse::<i64>() {
            Ok(x) => x,
            Err(err) => return Err(err.to_string()),
        };

        todo!()
    }

    fn delete(&mut self, name: &str) -> Option<Contact> {
        todo!()
    }

    fn get(&self, name: &str) -> Option<&Contact> {
        todo!()
    }

    fn list(&self, page_no: usize, page_size: usize) -> Vec<&Contact> {
        todo!()
    }

    fn export_to_json(&self, file_path: String) -> Result<(), std::io::Error> {
        todo!()
    }

    fn import_from_json(&mut self, path: String) -> Result<(), std::io::Error> {
        todo!()
    }

    fn count(&self) -> usize {
        todo!()
    }
}
