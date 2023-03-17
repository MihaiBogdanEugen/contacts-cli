use lazy_static::lazy_static;
use regex::Regex;
use std::collections::BTreeMap;

const EMAIL_REGEX_STR: &str =
    r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})";
const DE_PHONE_NO_REGEX_STR: &str = r"49[0-9]{9,10}";

pub struct Contact {
    pub name: String,
    pub phone_no: u64,
    pub email: String,
}

pub trait ContactsService {
    fn add(&mut self, name: String, phone_no_as_string: String, email: String) -> Result<(), &str>;

    fn update_email(&mut self, name: &str, new_email: String) -> Result<(), &str>;

    fn update_phone(&mut self, name: &str, new_phone_no_as_string: String) -> Result<(), &str>;

    fn delete(&mut self, name: &str) -> Option<Contact>;

    fn get(&self, name: &str) -> Option<&Contact>;

    fn list(&self, page_no: usize, page_size: usize) -> Vec<&Contact>;
}

pub struct InMemoryContactsService {
    contacts: BTreeMap<String, Contact>,
}

impl InMemoryContactsService {
    pub fn new() -> Self {
        InMemoryContactsService {
            contacts: BTreeMap::new(),
        }
    }

    fn is_valid_email(text: &String) -> bool {
        lazy_static! {
            static ref EMAIL_REGEX: Regex = Regex::new(EMAIL_REGEX_STR).unwrap();
        }
        EMAIL_REGEX.is_match(text)
    }

    fn is_valid_phone_no(text: &String) -> bool {
        lazy_static! {
            static ref DE_PHONE_NO_REGEX: Regex = Regex::new(DE_PHONE_NO_REGEX_STR).unwrap();
        }
        DE_PHONE_NO_REGEX.is_match(text)
    }
}

impl ContactsService for InMemoryContactsService {
    fn add(&mut self, name: String, phone_no_as_string: String, email: String) -> Result<(), &str> {
        if !Self::is_valid_email(&email) {
            return Err("invalid email");
        }

        if !Self::is_valid_phone_no(&phone_no_as_string) {
            return Err("invalid phone_no");
        }

        match phone_no_as_string.parse::<u64>() {
            Ok(phone_no) => {
                self.contacts.insert(
                    name.clone(),
                    Contact {
                        name,
                        phone_no,
                        email,
                    },
                );
                return Ok(());
            }
            Err(_) => return Err("phone_no is not a u64 value"),
        }
    }

    fn update_email(&mut self, name: &str, new_email: String) -> Result<(), &str> {
        if !Self::is_valid_email(&new_email) {
            return Err("invalid email");
        }

        match self.contacts.get_mut(name) {
            Some(contact) => {
                contact.email = new_email;
                return Ok(());
            }
            None => return Err("key not found"),
        }
    }

    fn update_phone(&mut self, name: &str, new_phone_no_as_string: String) -> Result<(), &str> {
        if !Self::is_valid_phone_no(&new_phone_no_as_string) {
            return Err("invalid phone_no");
        }

        match new_phone_no_as_string.parse::<u64>() {
            Ok(new_phone_no) => match self.contacts.get_mut(name) {
                Some(contact) => {
                    contact.phone_no = new_phone_no;
                    return Ok(());
                }
                None => return Err("key not found"),
            },
            Err(_) => return Err("phone_no is not a u64 value"),
        }
    }

    fn delete(&mut self, name: &str) -> Option<Contact> {
        return self.contacts.remove(name);
    }

    fn get(&self, name: &str) -> Option<&Contact> {
        return self.contacts.get(name);
    }

    fn list(&self, page_no: usize, page_size: usize) -> Vec<&Contact> {
        self.contacts
            .values()
            .skip(page_no * page_size)
            .take(page_size)
            .collect()
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

        let res_invalid_phone_no: Result<(), &str> = contacts_service.add(
            "valid name".to_string(),
            "invalid phone no".to_string(),
            "validemail@mail.com".to_string(),
        );
        assert!(res_invalid_phone_no.is_err());

        let res_invalid_email: Result<(), &str> = contacts_service.add(
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
                expected_phone_no_as_string.clone(),
                expected_email.clone(),
            )
            .unwrap();

        let new_expected_phone_no_as_string: String = "490123456789".to_string();
        contacts_service
            .update_phone("Bogdan", new_expected_phone_no_as_string.clone())
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
            .add(
                expected_name.clone(),
                expected_phone_no_as_string.clone(),
                expected_email.clone(),
            )
            .unwrap();

        contacts_service.delete("Bogdan").unwrap();

        let res_get = contacts_service.get("Bogdan");
        assert!(res_get.is_none());
    }

    #[test]
    fn test_in_memory_contacts_service_list() {
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
    }
}
