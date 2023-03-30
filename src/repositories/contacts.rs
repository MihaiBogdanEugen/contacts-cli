use crate::models::contact::Contact;
use regex::Regex;
use std::io::Error;

const EMAIL_REGEX: &str =
    r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})";
const DE_PHONE_NO_REGEX: &str = r"49[0-9]{9,10}";

pub fn is_valid_email(text: &str) -> Result<bool, regex::Error> {
    is_valid_regex(text, EMAIL_REGEX)
}

pub fn is_valid_phone_no(text: &str) -> Result<bool, regex::Error> {
    is_valid_regex(text, DE_PHONE_NO_REGEX)
}

fn is_valid_regex(text: &str, re: &str) -> Result<bool, regex::Error> {
    match Regex::new(re) {
        Ok(regex) => Ok(regex.is_match(text)),
        Err(err) => Err(err),
    }
}
pub trait ContactsRepository {
    fn add(
        &mut self,
        name: String,
        phone_no_as_string: String,
        email: String,
    ) -> Result<(), String>;

    fn update_email(&mut self, name: &str, new_email: String) -> Result<bool, String>;

    fn update_phone_no(
        &mut self,
        name: &str,
        new_phone_no_as_string: String,
    ) -> Result<bool, String>;

    fn delete(&mut self, name: &str) -> Option<Contact>;

    fn get(&self, name: &str) -> Option<&Contact>;

    fn list(&self, page_no: usize, page_size: usize) -> Vec<&Contact>;

    fn export_to_json(&self, file_path: String) -> Result<(), Error>;

    fn import_from_json(&mut self, path: String) -> Result<(), Error>;

    fn count(&self) -> usize;
}
