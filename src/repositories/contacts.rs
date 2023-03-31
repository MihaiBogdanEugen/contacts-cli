use crate::models::contact::Contact;
use regex::Regex;

const EMAIL_REGEX: &str =
    r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})";
const DE_PHONE_NO_REGEX: &str = r"49[0-9]{9,10}";

fn is_valid_regex(text: &str, re: &str) -> Result<bool, regex::Error> {
    match Regex::new(re) {
        Ok(regex) => Ok(regex.is_match(text)),
        Err(err) => Err(err),
    }
}

pub fn get_valid_name(name: &str) -> Result<String, String> {
    if name.is_empty() {
        return Err("name canot be empty".to_string());
    }

    return Ok(name.to_string());
}

pub fn get_valid_email(email: &str) -> Result<String, String> {

    let is_valid_email: bool = is_valid_regex(&email, EMAIL_REGEX)
        .map_err(|err| err.to_string())?;
    
    if !is_valid_email {
        return Err("Email is not valid".to_string());
    }

    return Ok(email.to_string());
}

pub fn get_valid_phone_no(phone_no_as_string: &str) -> Result<u64, String> {
    
    let is_valid_phone_no: bool = is_valid_regex(&phone_no_as_string, DE_PHONE_NO_REGEX)
        .map_err(|err| err.to_string())?;

    if !is_valid_phone_no {
        return Err("Phone no is not valid".to_string());
    }

    let phone_no: u64 = phone_no_as_string.parse::<u64>()
        .map_err(|err| err.to_string())?;

    return Ok(phone_no);
}

pub trait ContactsRepository {
    fn add(
        &mut self,
        name: String,
        phone_no_as_string: String,
        email: String,
    ) -> Result<(), String>;

    fn update_email(&mut self, name: &str, new_email: String) -> Result<(), String>;

    fn update_phone_no(
        &mut self,
        name: &str,
        new_phone_no_as_string: String,
    ) -> Result<(), String>;

    fn delete(&mut self, name: &str) -> Result<(), String>;

    fn get(&self, name: &str) -> Result<Option<&Contact>, String>;

    fn list(&self, page_no: usize, page_size: usize) -> Result<Vec<&Contact>, String>;

    fn export_to_json(&self, file_path: String) -> Result<(), String>;

    fn import_from_json(&mut self, path: String) -> Result<(), String>;

    fn count(&self) -> usize;
}
