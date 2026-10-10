#[derive(Debug)]
pub struct Email {
    id: String,
}

#[derive(Debug)]
enum EmailError {
    ValidationError,
}

impl Email {
    fn new(id: String) -> Result<Self, EmailError> {
        if id.contains('@') {
            return Ok(Email { id: id });
        }
        Err(EmailError::ValidationError)
    }
}

fn main() {
    let email = Email::new(String::from("test@gmail.com"));
    println!("{:?}", email);
}
