use regex::Regex;
use core::panic;
use std::collections::BTreeMap;

const EMAIL_REGEX: &str = r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})";
const DE_PHONE_NO_REGEX: &str = r"49[0-9]{9,10}";

struct Contact {
    name: String,
    phone_no: u64,
    email: String,
}

struct ContactsApp {
    contacts: BTreeMap<String, Contact>
}

impl ContactsApp {
    fn new() -> Self {
        ContactsApp { contacts: BTreeMap::new() }
    }

    fn add(&mut self, name: String, phone_no_as_string: String, email: String) -> Result<(), String> {
    
        let de_phone_no_regex: Regex; 
        match Regex::new(DE_PHONE_NO_REGEX) {
            Ok(r) => de_phone_no_regex = r,
            Err(_) => panic!("DE_PHONE_NO_REGEX is not a valid regex"),
        }

        if !de_phone_no_regex.is_match(&phone_no_as_string) {
            return Err(String::from("invalid phone_no"));
        }

        let phone_no;
        match phone_no_as_string.parse::<u64>() {
            Ok(val) => phone_no = val,
            Err(_) => panic!("valid phone number is not a u64 value"),
        }

        let email_regex: Regex;
        match Regex::new(EMAIL_REGEX) {
            Ok(r) => email_regex = r,
            Err(_) => panic!("EMAIL_REGEX is not a valid regex"),
        }

        if !email_regex.is_match(&email) {
            return Err(String::from("invalid email"));
        }

        self.contacts.insert(name.clone(), Contact { name, phone_no, email });
        Ok(())
    }

    fn update_email(&mut self, name: &str, new_email: String) -> Result<(), String> {

        let email_regex: Regex;
        match Regex::new(EMAIL_REGEX) {
            Ok(r) => email_regex = r,
            Err(_) => panic!("EMAIL_REGEX is not a valid regex"),
        }

        if !email_regex.is_match(&new_email) {
            return Err(String::from("invalid email"));
        }

        match self.contacts.get_mut(name) {
            Some(contact) => {
                contact.email = new_email;
                Ok(())
            }
            None => Err(String::from("unknown name key")),
        }
    }

    fn update_phone(&mut self, name: &str, new_phone_no_as_string: String) -> Result<(), String> {

        let de_phone_no_regex: Regex; 
        match Regex::new(DE_PHONE_NO_REGEX) {
            Ok(r) => de_phone_no_regex = r,
            Err(_) => panic!("DE_PHONE_NO_REGEX is not a valid regex"),
        }

        if !de_phone_no_regex.is_match(&new_phone_no_as_string) {
            return Err(String::from("invalid phone_no"));
        }

        let new_phone_no;
        match new_phone_no_as_string.parse::<u64>() {
            Ok(val) => new_phone_no = val,
            Err(_) => panic!("valid phone number is not a u64 value"),
        }

        match self.contacts.get_mut(name) {
            Some(contact) => {
                contact.phone_no = new_phone_no;
                Ok(())
            }
            None => Err("key not found".to_string()),
        }
    }

    fn delete(&mut self, name: &str) -> Result<(), String> {
        match self.contacts.remove(name) {
            Some(_) => Ok(()),
            None => Err("key not found".to_string()),
        }
    }

    fn get(&self, name: &str) -> Result<&Contact, String> {
        match self.contacts.get(name) {
            Some(contact) => Ok(contact),
            None => Err("key not found".to_string()),
        }
    }

    fn list(&self, page_no: usize, page_size: usize) -> Vec<&Contact> {
        self.contacts.values().skip(page_no * page_size).take(page_size).collect()
    }
}

fn main() {
    println!("Hello, world!");

    let mut contacts: BTreeMap<String, Contact> = BTreeMap::new();

    let name: String = String::from("Bogdan Mihai");
    let phone_no_as_string: String = String::from("491234567890");
    let email: String = String::from("bogdan.mihai@mail.com");

    let phone_no;
    match phone_no_as_string.parse::<u64>() {
        Ok(val) => phone_no = val,
        Err(_) => panic!("valid phone number is not a u64 value"),
    }

    let contact = Contact {
        name: name.clone(),
        phone_no,
        email: email.clone(),
    };

    contacts.insert(name.clone(), contact);
    assert!(contacts.contains_key(&name));

    match contacts.get(&name) {
        Some(actual_contact) => {
            assert_eq!(name, actual_contact.name);
            assert_eq!(phone_no, actual_contact.phone_no);
            assert_eq!(email, actual_contact.email);
        }
        None => panic!("there *must* be a contact with this name"),
    }

    let email_regex: Regex;
    match Regex::new(EMAIL_REGEX) {
        Ok(r) => email_regex = r,
        Err(_) => panic!("EMAIL_REGEX is not a valid regex"),
    }

    let de_phone_no_regex: Regex; 
    match Regex::new(DE_PHONE_NO_REGEX) {
        Ok(r) => de_phone_no_regex = r,
        Err(_) => panic!("DE_PHONE_NO_REGEX is not a valid regex"),
    }

    assert!(email_regex.is_match(&email));
    assert!(!email_regex.is_match(&"invalid_email"));

    assert!(de_phone_no_regex.is_match(&phone_no_as_string));
    assert!(!de_phone_no_regex.is_match(&"481234567890"));
    assert!(!de_phone_no_regex.is_match(&"49asda7890"));
    assert!(!de_phone_no_regex.is_match(&"asdafsdgs"));
}
