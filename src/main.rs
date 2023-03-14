use std::collections::BTreeMap;

struct Contact {
    name: String,
    phone_no: u64,
    email: String
}

fn main() {
    println!("Hello, world!");

    let mut contacts: BTreeMap<String, Contact> = BTreeMap::new();

    let name: String = String::from("Bogdan Mihai");
    let phone_no: u64 = 491234567890;
    let email: String = String::from("bogdan.mihai@mail.com");
    let contact = Contact { name: name.clone(), phone_no, email: email.clone() };
    
    contacts.insert(name.clone(), contact);
    assert!(contacts.contains_key(&name));

    match contacts.get(&name) {
        Some(actual_contact) => {
            assert_eq!(name, actual_contact.name);
            assert_eq!(phone_no, actual_contact.phone_no);
            assert_eq!(email, actual_contact.email);
        },
        None => panic!("there *must* be a contact with this name"),
    }
}