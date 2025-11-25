use std::io::{self, Write};

struct Contact {
    id: u32,
    number: String,
    name: String,
    email: Option<String>
}

impl Contact {
    fn display(&self) -> String {
        format!(
            "[{}] {} - {}{}",
            self.id, 
            self.name, 
            self.number, 
            match &self.email {
            Some(e) if !e.is_empty() => format!("{}", e),
            _ => "".into(),
        }
        )
    }
}


struct ContactBook {
    contacts : Vec<Contact>,
    next_id: u32,
}

impl ContactBook {
    fn new() -> Self {
        Self { contacts: Vec::new(), next_id: 1 }
    }

    fn add_contact(&mut self, name: String, number: String, email: Option<String>) {
        let contact = Contact {
            id: self.next_id,
            name,
            number,
            email
        };
        self.next_id += 1;
        self.contacts.push(contact);
        println!("contact added.")
    } 

    fn remove_by_token(&mut self, token: &str) -> bool {

        if let Ok(id) = token.trim().parse::<u32>()  {
            if let Some(pos) = self.contacts.iter().position(|c| c.id == id) {
                self.contacts.remove(pos);
                println!("Removed contact with id {}", id);
                return true;
            } else {
                println!("No contact found with id {}", id);
                return false;
            }
        }

        let token_l = token.to_lowercase();

        if let Some(pos) = self.contacts.iter().position(|c| c.name.to_lowercase().contains(&token_l)) {

            let removed = self.contacts.remove(pos);
            println!("Removed contact: {}", removed.display());
            return true;
        }


        println!("No contact found matching \"{}\"", token);
        false

    }
}


fn main() {
    let mut book = ContactBook::new();

    let _contact = Contact {
        id: 1,
        name: "Sachin".into(),
        number: "8398999896".into(),
        email: None,
        };

    book.add_contact("Sachin".into(), "8398999896".into(), None);
    book.add_contact("Deepak".into(), "9518059064".into(), None);

    // for i in 0..book.contacts.len() {
    //     println!("{}", book.contacts[i].display());
    // }

    // idiomatic rust 
    for contact in &book.contacts {
        println!("{}", contact.display());
    }
}
