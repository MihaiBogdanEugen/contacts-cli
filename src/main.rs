use crate::contacts_service::{Contact, ContactsService, InMemoryContactsService};

pub mod contacts_service;

fn main() {
    println!("Hello, world!");
    let mut contacts_service: InMemoryContactsService = InMemoryContactsService::new();
    contacts_service
        .add(
            "Bogdan".to_string(),
            "491234567890".to_string(),
            "bogdan@mail.com".to_string(),
        )
        .unwrap();
    let contact: &Contact = contacts_service.get("Bogdan").unwrap();
    println!(
        "Contact[{}, {}, {}]",
        contact.name, contact.phone_no, contact.email
    );
}
