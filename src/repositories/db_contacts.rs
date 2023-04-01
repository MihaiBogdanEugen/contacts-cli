use dotenvy::dotenv;
use redis::Client as RedisClient;
use redis::Connection as RedisConnection;
use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use crate::{models::contact::Contact, repositories::contacts::ContactsRepository};

use super::contacts::{get_valid_name, get_valid_email, get_valid_phone_no};

const REDIS_URL_KEY: &str = "REDIS_URL";
const REDIS_SUBKEY_PHONE_NO: &str = "phone_no";
const REDIS_SUBKEY_EMAIL: &str = "email";
const REDIS_KEY_PREFFIX: &str = "contacts";

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
            env::var(REDIS_URL_KEY).unwrap_or_else(|_| panic!("Cannot find key {REDIS_URL_KEY}"));
        let redis_client: RedisClient = RedisClient::open(redis_url.clone())
            .unwrap_or_else(|_| panic!("Cannot connect to Redis instance {redis_url}"));
        DbContactsRepository { redis_client }
    }

    fn get_redis_connection(&self) -> Result<RedisConnection, String> {
        let redis_connection: RedisConnection = self
            .redis_client
            .get_connection()
            .map_err(|err| err.to_string())?;

        Ok(redis_connection)
    }
}

impl ContactsRepository for DbContactsRepository {
    fn add(&mut self, name: String, phone_no_as_string: String, email: String) -> Result<(), String> {
        let name: String = get_valid_name(&name)?;
        let email:String = get_valid_email(&email)?;
        let phone_no: u64 = get_valid_phone_no(&phone_no_as_string)?;
        let mut redis_connection: RedisConnection = self.get_redis_connection()?;
        let key: String = format!("{REDIS_KEY_PREFFIX}:{name}");

        let no_of_subkeys_set: usize = redis::cmd("HSET").arg(&key)
            .arg(REDIS_SUBKEY_PHONE_NO)
            .arg(phone_no)
            .arg(REDIS_SUBKEY_EMAIL)
            .arg(email)        
            .query::<usize>(&mut redis_connection)
            .map_err(|err| err.to_string())?;

        if no_of_subkeys_set != 2 {
            return Err(format!("Unexpected Redis HSET result. Expected 2, actual {no_of_subkeys_set}"));
        }

        Ok(())
    }

    fn update_email(&mut self, name: &str, new_email: String) -> Result<(), String> {
        let new_email:String = get_valid_email(&new_email)?;
        let mut redis_connection: RedisConnection = self.get_redis_connection()?;
        let key: String = format!("{REDIS_KEY_PREFFIX}:{name}");

        let no_of_subkeys_set: usize = redis::cmd("HSET").arg(&key)
            .arg(REDIS_SUBKEY_EMAIL)
            .arg(new_email)        
            .query::<usize>(&mut redis_connection)
            .map_err(|err| err.to_string())?;

        if no_of_subkeys_set != 1 {
            return Err(format!("Unexpected Redis HSET result. Expected 1, actual {no_of_subkeys_set}"));
        }
        
        Ok(())
    }

    fn update_phone_no(&mut self, name: &str, new_phone_no_as_string: String) -> Result<(), String> {
        let new_phone_no: u64 = get_valid_phone_no(&new_phone_no_as_string)?;
        let mut redis_connection: RedisConnection = self.get_redis_connection()?;
        let key: String = format!("{REDIS_KEY_PREFFIX}:{name}");

        let no_of_subkeys_set: usize = redis::cmd("HSET").arg(&key)
            .arg(REDIS_SUBKEY_PHONE_NO)
            .arg(new_phone_no)
            .query::<usize>(&mut redis_connection)
            .map_err(|err| err.to_string())?;

        if no_of_subkeys_set != 1 {
            return Err(format!("Unexpected Redis HSET result. Expected 1, actual {no_of_subkeys_set}"));
        }

        Ok(())
    }

    fn delete(&mut self, name: &str) -> Result<(), String> {
        let mut redis_connection: RedisConnection = self.get_redis_connection()?;
        let key: String = format!("{REDIS_KEY_PREFFIX}:{name}");

        let no_of_subkeys_set: usize = redis::cmd("DEL").arg(&key)
            .query::<usize>(&mut redis_connection)
            .map_err(|err| err.to_string())?;

        if no_of_subkeys_set != 1 {
            return Err(format!("Unexpected Redis DEL result. Expected 1, actual {no_of_subkeys_set}"));
        }

        Ok(())
    }

    fn get(&self, name: &str) -> Result<Option<Contact>, String> {
        let mut redis_connection: RedisConnection = self.get_redis_connection()?;
        let key: String = format!("{REDIS_KEY_PREFFIX}:{name}");

        let values: Vec<String> = redis::cmd("HGETALL").arg(&key)
            .query::<Vec<String>>(&mut redis_connection)
            .unwrap();  

        if values.len() % 4 != 0 {
            return Err("Invalid response received from Redis, cannot construct key-value pairs list".to_string());
        }
        
        let phone_no_as_string: &String = &values[1];
        let phone_no: u64 = phone_no_as_string.parse::<u64>()
            .map_err(|err| err.to_string())?;
        let email: &String = &values[3];

        let contact: Contact = Contact { name: name.to_string(), phone_no, email: email.to_string() };
        Ok(Some(contact))
    }

    fn list(&self, page_no: usize, page_size: usize) -> Result<Vec<Contact>, String> {
        let mut redis_connection: RedisConnection = self.get_redis_connection()?;

        let keys: Vec<String> = redis::cmd("KEYS")
            .arg(format!("{REDIS_KEY_PREFFIX}:*"))
            .query::<Vec<String>>(&mut redis_connection)
            .map_err(|err| err.to_string())?;

        let mut map: BTreeMap<String, Contact> = BTreeMap::new();

        for mut key in keys {
            
            let values: Vec<String> = redis::cmd("HGETALL").arg(&key)
            .query::<Vec<String>>(&mut redis_connection)
            .unwrap();  
    
            let index = key.find(':').unwrap_or(key.len());
            let name: String = key.drain(1+index..).collect();

            if values.len() % 4 != 0 {
                return Err("Invalid response received from Redis, cannot construct key-value pairs list".to_string());
            }
            
            let phone_no_as_string: &String = &values[1];
            let phone_no: u64 = phone_no_as_string.parse::<u64>()
                .map_err(|err| err.to_string())?;
            let email: &String = &values[3];
    
            let contact: Contact = Contact { name: name.to_string(), phone_no, email: email.to_string() };

            map.insert(name, contact);
        }

        let contacts: Vec<Contact> = map
            .values()
            .skip(page_no * page_size)
            .take(page_size)
            .map(|c| (*c).clone())
            .collect();

        Ok(contacts)
    }

    fn export_to_json(&self, path: String) -> Result<(), String> {
        let mut redis_connection: RedisConnection = self.get_redis_connection()?;

        let keys: Vec<String> = redis::cmd("KEYS")
            .arg(format!("{REDIS_KEY_PREFFIX}:*"))
            .query::<Vec<String>>(&mut redis_connection)
            .map_err(|err| err.to_string())?;

        let mut map: BTreeMap<String, Contact> = BTreeMap::new();

        for mut key in keys {
            
            let values: Vec<String> = redis::cmd("HGETALL").arg(&key)
            .query::<Vec<String>>(&mut redis_connection)
            .unwrap();  
    
            let index = key.find(':').unwrap_or(key.len());
            let name: String = key.drain(1+index..).collect();

            if values.len() % 4 != 0 {
                return Err("Invalid response received from Redis, cannot construct key-value pairs list".to_string());
            }
            
            let phone_no_as_string: &String = &values[1];
            let phone_no: u64 = phone_no_as_string.parse::<u64>()
                .map_err(|err| err.to_string())?;
            let email: &String = &values[3];
    
            let contact: Contact = Contact { name: name.to_string(), phone_no, email: email.to_string() };

            map.insert(name, contact);
        }

        let contacts: Vec<Contact> = map
            .values()
            .map(|c| (*c).clone())
            .collect();

        let json_str: String = serde_json::to_string(&contacts)
            .map_err(|err| err.to_string())?;
        let mut file: File = File::create(path)
            .map_err(|err| err.to_string())?;
        file.write_all(json_str.as_bytes())
            .map_err(|err| err.to_string())?;
        Ok(())
    
    }

    fn import_from_json(&mut self, path: String) -> Result<(), String> {
        let inner: File = File::open(path)
            .map_err(|err| err.to_string())?;
        let rdr: BufReader<File> = BufReader::new(inner);
        let contacts: Vec<Contact> = serde_json::from_reader(rdr)
            .map_err(|err| err.to_string())?;

        let mut redis_connection: RedisConnection = self.get_redis_connection()?;

        for contact in contacts {
            
            let key: String = format!("{REDIS_KEY_PREFFIX}:{}", contact.name);

            let no_of_subkeys_set: usize = redis::cmd("HSET").arg(&key)
                .arg(REDIS_SUBKEY_PHONE_NO)
                .arg(contact.phone_no)
                .arg(REDIS_SUBKEY_EMAIL)
                .arg(contact.email)        
                .query::<usize>(&mut redis_connection)
                .map_err(|err| err.to_string())?;

            if no_of_subkeys_set != 2 {
                return Err(format!("Unexpected Redis HSET result. Expected 2, actual {no_of_subkeys_set}"));
            }
        }

        Ok(())
    }

    fn count(&self) -> Result<usize, String> {
        let mut redis_connection: RedisConnection = self.get_redis_connection()?;

        let keys: Vec<String> = redis::cmd("KEYS")
            .arg(format!("{REDIS_KEY_PREFFIX}:*"))
            .query::<Vec<String>>(&mut redis_connection)
            .map_err(|err| err.to_string())?;

        Ok(keys.len())
    }
}
