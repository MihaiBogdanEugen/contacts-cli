use regex::Regex;
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufReader, Error, Write},
};

use crate::models::contact::Contact;

const EMAIL_REGEX: &str =
    r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})";
const DE_PHONE_NO_REGEX: &str = r"49[0-9]{9,10}";

pub trait ContactsService {
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

pub struct InMemoryContactsService {
    contacts: BTreeMap<String, Contact>,
}

impl Default for InMemoryContactsService {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryContactsService {
    pub fn new() -> Self {
        InMemoryContactsService {
            contacts: BTreeMap::new(),
        }
    }

    fn is_valid_email(text: &str) -> Result<bool, regex::Error> {
        Self::is_valid_regex(text, EMAIL_REGEX)
    }

    fn is_valid_phone_no(text: &str) -> Result<bool, regex::Error> {
        Self::is_valid_regex(text, DE_PHONE_NO_REGEX)
    }

    fn is_valid_regex(text: &str, re: &str) -> Result<bool, regex::Error> {
        match Regex::new(re) {
            Ok(regex) => Ok(regex.is_match(text)),
            Err(err) => Err(err),
        }
    }
}

impl ContactsService for InMemoryContactsService {
    fn add(
        &mut self,
        name: String,
        phone_no_as_string: String,
        email: String,
    ) -> Result<(), String> {
        if name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }

        match Self::is_valid_email(&email) {
            Ok(is_valid_email) => {
                if !is_valid_email {
                    return Err("Email is not valid".to_string());
                }
            }
            Err(err) => return Err(err.to_string()),
        }

        match Self::is_valid_phone_no(&phone_no_as_string) {
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

        self.contacts.insert(
            name.clone(),
            Contact {
                name,
                phone_no,
                email,
            },
        );
        Ok(())
    }

    fn update_email(&mut self, name: &str, new_email: String) -> Result<bool, String> {
        match Self::is_valid_email(&new_email) {
            Ok(is_valid_email) => {
                if !is_valid_email {
                    return Err("New email is not valid".to_string());
                }
            }
            Err(err) => return Err(err.to_string()),
        }

        let contact: &mut Contact = match self.contacts.get_mut(name) {
            Some(x) => x,
            None => return Ok(false),
        };

        contact.email = new_email;
        Ok(true)
    }

    fn update_phone_no(
        &mut self,
        name: &str,
        new_phone_no_as_string: String,
    ) -> Result<bool, String> {
        match Self::is_valid_phone_no(&new_phone_no_as_string) {
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

        let contact: &mut Contact = match self.contacts.get_mut(name) {
            Some(x) => x,
            None => return Ok(false),
        };

        contact.phone_no = new_phone_no;
        Ok(true)
    }

    fn delete(&mut self, name: &str) -> Option<Contact> {
        self.contacts.remove(name)
    }

    fn get(&self, name: &str) -> Option<&Contact> {
        self.contacts.get(name)
    }

    fn list(&self, page_no: usize, page_size: usize) -> Vec<&Contact> {
        self.contacts
            .values()
            .skip(page_no * page_size)
            .take(page_size)
            .collect()
    }

    fn count(&self) -> usize {
        return self.contacts.values().count();
    }

    fn export_to_json(&self, path: String) -> Result<(), Error> {
        let list: Vec<&Contact> = self.contacts.values().collect();
        let json_str: String = serde_json::to_string(&list)?;
        let mut file: File = File::create(path)?;
        file.write_all(json_str.as_bytes())?;
        Ok(())
    }

    fn import_from_json(&mut self, path: String) -> Result<(), Error> {
        let inner: File = File::open(path)?;
        let rdr: BufReader<File> = BufReader::new(inner);
        let contacts: Vec<Contact> = serde_json::from_reader(rdr).unwrap();
        for contact in contacts {
            self.contacts.insert(contact.name.clone(), contact);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_memory_contacts_service_add_get() {
        let mut contacts_service: InMemoryContactsService = InMemoryContactsService::new();

        let expected_name: String = "Bogdan".to_string();
        let expected_phone_no_as_string: String = "491234567890".to_string();
        let expected_email: String = "bogdan@mail.com".to_string();

        contacts_service
            .add(
                expected_name.clone(),
                expected_phone_no_as_string.clone(),
                expected_email.clone(),
            )
            .unwrap();

        let actual_contact: &Contact = contacts_service.get("Bogdan").unwrap();

        assert_eq!(expected_name, actual_contact.name);
        assert_eq!(
            expected_phone_no_as_string,
            actual_contact.phone_no.to_string()
        );
        assert_eq!(expected_email, actual_contact.email);
    }

    #[test]
    fn test_in_memory_contacts_service_add_validations() {
        let mut contacts_service: InMemoryContactsService = InMemoryContactsService::new();

        let res_invalid_phone_no: Result<(), String> = contacts_service.add(
            "valid name".to_string(),
            "invalid phone no".to_string(),
            "validemail@mail.com".to_string(),
        );
        assert!(res_invalid_phone_no.is_err());

        let res_invalid_email: Result<(), String> = contacts_service.add(
            "valid name".to_string(),
            "491234567890".to_string(),
            "invalid email".to_string(),
        );
        assert!(res_invalid_email.is_err());
    }

    #[test]
    fn test_in_memory_contacts_service_updates() {
        let mut contacts_service: InMemoryContactsService = InMemoryContactsService::new();

        let expected_name: String = "Bogdan".to_string();
        let expected_phone_no_as_string: String = "491234567890".to_string();
        let expected_email: String = "bogdan@mail.com".to_string();

        contacts_service
            .add(
                expected_name.clone(),
                expected_phone_no_as_string,
                expected_email,
            )
            .unwrap();

        let new_expected_phone_no_as_string: String = "490123456789".to_string();
        contacts_service
            .update_phone_no("Bogdan", new_expected_phone_no_as_string.clone())
            .unwrap();

        let new_email: String = "new_bogdan@mail.com".to_string();
        contacts_service
            .update_email("Bogdan", new_email.clone())
            .unwrap();

        let actual_contact: &Contact = contacts_service.get("Bogdan").unwrap();
        assert_eq!(expected_name, actual_contact.name);
        assert_eq!(
            new_expected_phone_no_as_string,
            actual_contact.phone_no.to_string()
        );
        assert_eq!(new_email, actual_contact.email);
    }

    #[test]
    fn test_in_memory_contacts_service_delete() {
        let mut contacts_service: InMemoryContactsService = InMemoryContactsService::new();

        let expected_name: String = "Bogdan".to_string();
        let expected_phone_no_as_string: String = "491234567890".to_string();
        let expected_email: String = "bogdan@mail.com".to_string();

        contacts_service
            .add(expected_name, expected_phone_no_as_string, expected_email)
            .unwrap();

        contacts_service.delete("Bogdan").unwrap();

        let res_get = contacts_service.get("Bogdan");
        assert!(res_get.is_none());
    }

    #[test]
    fn test_in_memory_contacts_service_list_count() {
        let mut contacts_service: InMemoryContactsService = InMemoryContactsService::new();
        contacts_service
            .add(
                "Eee".to_string(),
                "491234567890".to_string(),
                "mail@mail.com".to_string(),
            )
            .unwrap();
        contacts_service
            .add(
                "Mmm".to_string(),
                "491234567890".to_string(),
                "mail@mail.com".to_string(),
            )
            .unwrap();
        contacts_service
            .add(
                "Bbb".to_string(),
                "491234567890".to_string(),
                "mail@mail.com".to_string(),
            )
            .unwrap();
        contacts_service
            .add(
                "Ddd".to_string(),
                "491234567890".to_string(),
                "mail@mail.com".to_string(),
            )
            .unwrap();
        contacts_service
            .add(
                "Sss".to_string(),
                "491234567890".to_string(),
                "mail@mail.com".to_string(),
            )
            .unwrap();
        contacts_service
            .add(
                "Aaa".to_string(),
                "491234567890".to_string(),
                "mail@mail.com".to_string(),
            )
            .unwrap();
        contacts_service
            .add(
                "Eee".to_string(),
                "491234567890".to_string(),
                "mail@mail.com".to_string(),
            )
            .unwrap();
        contacts_service
            .add(
                "Lll".to_string(),
                "491234567890".to_string(),
                "mail@mail.com".to_string(),
            )
            .unwrap();
        contacts_service
            .add(
                "Ccc".to_string(),
                "491234567890".to_string(),
                "mail@mail.com".to_string(),
            )
            .unwrap();
        contacts_service
            .add(
                "Aaa2".to_string(),
                "491234567890".to_string(),
                "mail@mail.com".to_string(),
            )
            .unwrap();
        contacts_service
            .add(
                "Aaa3".to_string(),
                "491234567890".to_string(),
                "mail@mail.com".to_string(),
            )
            .unwrap();

        let page0: Vec<&Contact> = contacts_service.list(0, 3);
        assert_eq!(3, page0.len());
        assert_eq!("Aaa", page0.get(0).unwrap().name);
        assert_eq!("Aaa2", page0.get(1).unwrap().name);
        assert_eq!("Aaa3", page0.get(2).unwrap().name);

        let page1: Vec<&Contact> = contacts_service.list(1, 3);
        assert_eq!(3, page1.len());
        assert_eq!("Bbb", page1.get(0).unwrap().name);
        assert_eq!("Ccc", page1.get(1).unwrap().name);
        assert_eq!("Ddd", page1.get(2).unwrap().name);

        let page2: Vec<&Contact> = contacts_service.list(2, 3);
        assert_eq!(3, page2.len());
        assert_eq!("Eee", page2.get(0).unwrap().name);
        assert_eq!("Lll", page2.get(1).unwrap().name);
        assert_eq!("Mmm", page2.get(2).unwrap().name);

        let page3: Vec<&Contact> = contacts_service.list(3, 3);
        assert_eq!(1, page3.len());
        assert_eq!("Sss", page3.get(0).unwrap().name);

        assert_eq!(10, contacts_service.count());
    }
}
