use std::io::{self, Write};

struct Contact {
    id: u8,
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
    next_id: u8,
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

    for i in 0..book.contacts.len() {
        println!("{}", book.contacts[i].display());
    }
}
