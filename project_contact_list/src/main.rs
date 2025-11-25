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
    contact : Vec<Contact>,
    next_id: u8,
}

impl ContactBook {
    fn new() -> Self {
        Self { contact: Vec::new(), next_id: 1 }
    }
}


fn main() {
    let contact_book = ContactBook::new();

    let contact = Contact {
        id: 1,
        name: "Sachin".into(),
        number: "8398999896".into(),
        email: None,
        };

        println!("{}", contact.display());
}
