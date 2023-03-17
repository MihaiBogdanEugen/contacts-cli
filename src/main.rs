use crate::contacts_service::{ContactsService, InMemoryContactsService};
use clap::{arg, ArgMatches, Command};
use std::io::Write;

pub mod contacts_service;

fn main() -> Result<(), String> {
    println!("contacts-app");
    let mut contacts_service: InMemoryContactsService = InMemoryContactsService::new();

    loop {
        let line: String = readline()?;
        let line: &str = line.trim();
        if line.is_empty() {
            continue;
        }

        match respond(line, &mut contacts_service) {
            Ok(quit) => {
                if quit {
                    break;
                }
            }
            Err(err) => {
                write!(std::io::stdout(), "{err}").map_err(|e| e.to_string())?;
                std::io::stdout().flush().map_err(|e| e.to_string())?;
            }
        }
    }

    Ok(())
}

fn respond(line: &str, contacts_service: &mut InMemoryContactsService) -> Result<bool, String> {
    let args: Vec<String> = shlex::split(line).ok_or("error: Invalid quoting")?;

    let matches: ArgMatches = cli()
        .try_get_matches_from(args)
        .map_err(|e| e.to_string())?;

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let name: &String = sub_matches.get_one::<String>("NAME").expect("required");
            let phone_no_as_string: &String =
                sub_matches.get_one::<String>("PHONE_NO").expect("required");
            let email: &String = sub_matches.get_one::<String>("EMAIL").expect("required");

            match contacts_service.add(
                name.to_string(),
                phone_no_as_string.to_string(),
                email.to_string(),
            ) {
                Ok(_) => println!("Contact added succesfully"),
                Err(msg) => println!("Err: {}", msg),
            }
        }
        Some(("view", sub_matches)) => {
            let name: &String = sub_matches.get_one::<String>("NAME").expect("required");

            match contacts_service.get(name) {
                Some(contact) => println!(
                    "Contact\n- name: {}\n- phone_no: {}\n- email: {}\n",
                    contact.name, contact.phone_no, contact.email
                ),
                None => println!("No contact with name {}", name),
            }
        }
        Some(("update-phone-no", sub_matches)) => {
            let name: &String = sub_matches.get_one::<String>("NAME").expect("required");
            let new_phone_no_as_string: &String = sub_matches
                .get_one::<String>("NEW_PHONE_NO")
                .expect("required");

            match contacts_service.update_phone_no(name, new_phone_no_as_string.to_string()) {
                Ok(_) => println!("Contact updated succesfully"),
                Err(msg) => println!("Err: {}", msg),
            }
        }
        Some(("update-email", sub_matches)) => {
            let name: &String = sub_matches.get_one::<String>("NAME").expect("required");
            let new_email: &String = sub_matches
                .get_one::<String>("NEW_EMAIL")
                .expect("required");

            match contacts_service.update_email(name, new_email.to_string()) {
                Ok(_) => println!("Contact updated succesfully"),
                Err(msg) => println!("Err: {}", msg),
            }
        }
        Some(("delete", sub_matches)) => {
            let name: &String = sub_matches.get_one::<String>("NAME").expect("required");

            match contacts_service.delete(name) {
                Some(_) => println!("Contact deleted succesfully"),
                None => println!("No contact with name {}", name),
            }
        }
        Some(("quit", _matches)) => {
            write!(std::io::stdout(), "Exiting ...").map_err(|e| e.to_string())?;
            std::io::stdout().flush().map_err(|e| e.to_string())?;
            return Ok(true);
        }
        Some((name, _matches)) => println!("unknown command `{}`", name),
        None => unreachable!("subcommand required"),
    }

    Ok(false)
}

fn cli() -> Command {
    Command::new("contacts-app")
        .about("Small & primitive contacts application with a REPL CLI")
        .multicall(true)
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(
            Command::new("add")
                .about("Add a new contact")
                .arg(arg!(<NAME> "The name of the contact"))
                .arg(arg!(<PHONE_NO> "The phone_no of the contact"))
                .arg(arg!(<EMAIL> "The email of the contact"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("view")
                .about("View a new contact")
                .arg(arg!(<NAME> "The name of the contact"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("update-phone-no")
                .about("Update the phone_no of a contact")
                .arg(arg!(<NAME> "The name of the contact"))
                .arg(arg!(<NEW_PHONE_NO> "The new phone_no of the contact"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("update-email")
                .about("Update the email of a contact")
                .arg(arg!(<NAME> "The name of the contact"))
                .arg(arg!(<NEW_EMAIL> "The new email of the contact"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("delete")
                .about("Delete a new contact")
                .arg(arg!(<NAME> "The name of the contact"))
                .arg_required_else_help(true),
        )
        .subcommand(Command::new("quit").alias("exit").about("Quit the REPL"))
}

fn readline() -> Result<String, String> {
    write!(std::io::stdout(), "$ ").map_err(|e| e.to_string())?;
    std::io::stdout().flush().map_err(|e| e.to_string())?;
    let mut buffer: String = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}
