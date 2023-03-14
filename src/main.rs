use regex::Regex;
use std::collections::BTreeMap;

struct Contact {
    name: String,
    phone_no: u64,
    email: String,
}

const EMAIL_REGEX: &str = r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})";
const DE_PHONE_NO_REGEX: &str = r"49[0-9]{9,10}";

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

    let email_regex: Regex = Regex::new(EMAIL_REGEX).unwrap();
    let de_phone_no_regex: Regex = Regex::new(DE_PHONE_NO_REGEX).unwrap();

    assert!(email_regex.is_match(&email));
    assert!(!email_regex.is_match(&"invalid_email"));

    assert!(de_phone_no_regex.is_match(&phone_no_as_string));
    assert!(!de_phone_no_regex.is_match(&"481234567890"));
    assert!(!de_phone_no_regex.is_match(&"49asda7890"));
    assert!(!de_phone_no_regex.is_match(&"asdafsdgs"));
}
