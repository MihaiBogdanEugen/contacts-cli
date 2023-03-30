use std::io::Error;

use crate::models::contact::Contact;


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
