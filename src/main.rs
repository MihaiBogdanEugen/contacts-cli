use clap::{arg, ArgMatches, Command};
use models::contact::Contact;
use repositories::{inmemory_contacts::InMemoryContactsRepository, contacts::ContactsRepository};
use std::io::Write;

mod db;
mod models;
mod repositories;

fn main() -> Result<(), String> {
    stdout_write(
        "contacts-app\n\nUse `help` to discover more commands, or `quit` to exit the REPL\n",
    )?;
    let mut contacts_service: InMemoryContactsRepository = InMemoryContactsRepository::new();

    loop {
        let no_of_contacts: usize = contacts_service.count();
        let line: String = stdin_read_line(no_of_contacts)?;
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
                stderr_write(err.as_str())?;
                stderr_flush()?;
            }
        }
    }

    Ok(())
}

fn respond(line: &str, contacts_service: &mut InMemoryContactsRepository) -> Result<bool, String> {
    let args: Vec<String> = shlex::split(line).ok_or("error: Invalid quoting")?;
    let matches: ArgMatches = cli()
        .try_get_matches_from(args)
        .map_err(|e| e.to_string())?;
    let mut quit: bool = false;

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let name: String = get_arg("NAME", sub_matches).to_string();
            let phone_no_as_string: String = get_arg("PHONE_NO", sub_matches).to_string();
            let email: String = get_arg("EMAIL", sub_matches).to_string();
            contacts_service.add(name, phone_no_as_string, email)?;
            stdout_write("Contact added succesfully")?;
        }
        Some(("update-phone-no", sub_matches)) => {
            let name: &str = get_arg("NAME", sub_matches);
            let new_phone_no_as_string: String = get_arg("NEW_PHONE_NO", sub_matches).to_string();
            if contacts_service.update_phone_no(name, new_phone_no_as_string)? {
                stdout_write("Contact updated succesfully")?;
            } else {
                stdout_write_unknown_key(name)?;
            }
        }
        Some(("update-email", sub_matches)) => {
            let name: &str = get_arg("NAME", sub_matches);
            let new_email: String = get_arg("NEW_EMAIL", sub_matches).to_string();
            if contacts_service.update_email(name, new_email)? {
                stdout_write("Contact updated succesfully")?;
            } else {
                stdout_write_unknown_key(name)?;
            }
        }
        Some(("view", sub_matches)) => {
            let name: &str = get_arg("NAME", sub_matches);
            match contacts_service.get(name) {
                Some(contact) => stdout_write_contact(contact)?,
                None => stdout_write_unknown_key(name)?,
            }
        }
        Some(("delete", sub_matches)) => {
            let name: &str = get_arg("NAME", sub_matches);
            match contacts_service.delete(name) {
                Some(_) => stdout_write("Contact deleted succesfully")?,
                None => stdout_write_unknown_key(name)?,
            }
        }
        Some(("export", sub_matches)) => {
            let path: &str = get_arg("PATH", sub_matches);
            match contacts_service.export_to_json(path.to_string()) {
                Ok(_) => stdout_write("Contacts exported successfully")?,
                Err(err) => stderr_write(&err.to_string())?,
            }
        }
        Some(("import", sub_matches)) => {
            let path: &str = get_arg("PATH", sub_matches);
            match contacts_service.import_from_json(path.to_string()) {
                Ok(_) => stdout_write("Contacts imported successfully")?,
                Err(err) => stderr_write(&err.to_string())?,
            }
        }
        Some(("list", sub_matches)) => {
            let page_no_as_str: &str = get_arg("PAGE_NO", sub_matches);
            let page_size_as_str: &str = get_arg("PAGE_SIZE", sub_matches);

            let page_no: usize = page_no_as_str.parse::<usize>().unwrap_or(0);
            let page_size: usize = page_size_as_str.parse::<usize>().unwrap_or(10);
            let contacts: Vec<&Contact> = contacts_service.list(page_no, page_size);

            stdout_write_contacts(contacts)?;
        }
        Some(("quit", _)) => {
            stdout_write("Exiting...")?;
            quit = true;
        }
        Some((command, _)) => {
            stderr_write_unknown_command(command)?;
            stderr_flush()?;
        }
        None => unreachable!("subcommand required"),
    }

    stdout_flush()?;
    Ok(quit)
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
        .subcommand(
            Command::new("export")
                .about("Export contacts to a json file")
                .arg(arg!(<PATH> "The path of the json file"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("import")
                .about("Import contacts from a json file")
                .arg(arg!(<PATH> "The path of the json file"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("list")
                .about("List contacts")
                .arg(arg!(<PAGE_NO> "Page no."))
                .arg(arg!(<PAGE_SIZE> "Page size"))
                .arg_required_else_help(true),
        )
        .subcommand(Command::new("quit").alias("exit").about("Quit the REPL"))
}

fn stdin_read_line(no_of_contacts: usize) -> Result<String, String> {
    stdout_write_prompt(no_of_contacts)?;
    stdout_flush()?;
    let mut buf: String = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .map_err(|e| e.to_string())?;
    Ok(buf)
}

fn stdout_flush() -> Result<(), String> {
    std::io::stdout().flush().map_err(|e| e.to_string())
}

fn stderr_flush() -> Result<(), String> {
    std::io::stderr().flush().map_err(|e| e.to_string())
}

fn stdout_write(text: &str) -> Result<(), String> {
    write!(std::io::stdout(), "{}", text).map_err(|e| e.to_string())
}

fn stdout_write_prompt(no_of_contacts: usize) -> Result<(), String> {
    let suffix = if no_of_contacts == 1 { "" } else { "s" };
    write!(
        std::io::stdout(),
        "\n{} contact{} currently in the data store.\n\n$ ",
        no_of_contacts,
        suffix
    )
    .map_err(|e| e.to_string())
}

fn stdout_write_contact(contact: &Contact) -> Result<(), String> {
    write!(
        std::io::stdout(),
        "Contact\n- name: {}\n- phone_no: {}\n- email: {}",
        contact.name,
        contact.phone_no,
        contact.email
    )
    .map_err(|e| e.to_string())
}

fn stdout_write_contacts(contacts: Vec<&Contact>) -> Result<(), String> {
    for contact in contacts {
        stdout_write("-------------")?;
        stdout_write_contact(contact)?;
        stdout_write("-------------")?;
    }
    Ok(())
}

fn stdout_write_unknown_key(key: &str) -> Result<(), String> {
    write!(std::io::stdout(), "No contact with name {}", key).map_err(|e| e.to_string())
}

fn stderr_write(err: &str) -> Result<(), String> {
    write!(std::io::stderr(), "Err: {}", err).map_err(|e| e.to_string())
}

fn stderr_write_unknown_command(command: &str) -> Result<(), String> {
    write!(std::io::stderr(), "Unknown command: {}", command).map_err(|e| e.to_string())
}

fn get_arg<'a>(id: &str, sub_matches: &'a ArgMatches) -> &'a str {
    sub_matches.get_one::<String>(id).expect("required")
}
