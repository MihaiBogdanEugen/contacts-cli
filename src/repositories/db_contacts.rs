use dotenvy::dotenv;
use redis::Client as RedisClient;
use redis::Commands;
use redis::Connection as RedisConnection;
use std::env;

use crate::{models::contact::Contact, repositories::contacts::ContactsRepository};

use super::contacts::{get_valid_name, get_valid_email, get_valid_phone_no};

const REDIS_URL_KEY: &str = "REDIS_URL";
pub struct DbContactsRepository {
    redis_client: RedisClient,
}

impl Default for DbContactsRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl DbContactsRepository {
    pub fn new() -> Self {
        dotenv().expect("Error loading .env file");
        let redis_url: String =
            env::var(REDIS_URL_KEY).expect(format!("Cannot find key {REDIS_URL_KEY}").as_str());
        let redis_client: RedisClient = RedisClient::open(redis_url.clone())
            .expect(format!("Cannot connect to Redis instance {redis_url}").as_str());
        DbContactsRepository { redis_client }
    }
}

impl ContactsRepository for DbContactsRepository {
    fn add(
        &mut self,
        name: String,
        phone_no_as_string: String,
        email: String,
    ) -> Result<(), String> {
        let name: String = get_valid_name(&name)?;
        let email:String = get_valid_email(&email)?;
        let phone_no: u64 = get_valid_phone_no(&phone_no_as_string)?;
        let mut redis_connection: RedisConnection = self
            .redis_client
            .get_connection()
            .map_err(|err| err.to_string())?;
        let key: String = format!("contacts:{name}");

        todo!()
    }

    fn update_email(&mut self, name: &str, new_email: String) -> Result<(), String> {
        let email:String = get_valid_email(&new_email)?;
        let mut redis_connection: RedisConnection = self
            .redis_client
            .get_connection()
            .map_err(|err| err.to_string())?;
        let key: String = format!("contacts:{name}");

        todo!()
    }

    fn update_phone_no(
        &mut self,
        name: &str,
        new_phone_no_as_string: String,
    ) -> Result<(), String> {
        let new_phone_no: u64 = get_valid_phone_no(&new_phone_no_as_string)?;
        let mut redis_connection: RedisConnection = self
            .redis_client
            .get_connection()
            .map_err(|err| err.to_string())?;
        let key: String = format!("contacts:{name}");

        todo!()
    }

    fn delete(&mut self, name: &str) -> Result<(), String> {
        let mut redis_connection: RedisConnection = self
            .redis_client
            .get_connection()
            .map_err(|err| err.to_string())?;
        let key: String = format!("contacts:{name}");

        redis_connection.del(key)
            .map_err(|err| err.to_string())?;

        Ok(())
    }

    fn get(&self, name: &str) -> Result<Option<&Contact>, String> {
        let mut redis_connection: RedisConnection = self
            .redis_client
            .get_connection()
            .map_err(|err| err.to_string())?;
        let key: String = format!("contacts:{name}");

        let values: Vec<String> = redis_connection.hgetall(key)
            .map_err(|err| err.to_string())?;        

        todo!()
    }

    fn list(&self, page_no: usize, page_size: usize) -> Result<Vec<&Contact>, String> {
        todo!()
    }

    fn export_to_json(&self, file_path: String) -> Result<(), String> {
        todo!()
    }

    fn import_from_json(&mut self, path: String) -> Result<(), String> {
        todo!()
    }

    fn count(&self) -> usize {
        todo!()
    }
}
