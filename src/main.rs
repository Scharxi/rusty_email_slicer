use std::ops::Deref;

pub struct Email(String);

impl Deref for Email {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Email {
    /// Returns whether the given string is an email by checking
    /// if it contains an '@'
    fn is_email(&self) -> bool {
        self.contains('@')
    }

    /// Returns the provider
    pub fn get_provider(&self) -> &'_ str {
        assert!(!self.is_empty());
        assert!(self.is_email());
        self.trim().split('@').collect::<Vec<&str>>()[1]
    }

    /// Returns the user
    pub fn get_user(&self) -> &'_ str {
        assert!(!self.is_empty());
        assert!(self.is_email());
        self.trim().split('@').collect::<Vec<&str>>()[0]
    }
}

fn main() {
    let mail = Email("hello.world@outlook.de".to_string());
    let provider = mail.get_provider();
    let user = mail.get_user();
    println!("Your username is {user} and the provider is {provider}");
}


#[cfg(test)]
mod tests {
    use crate::Email;

    #[test]
    #[should_panic]
    fn test_valid_email_format() {
        let invalid_mail = Email("hello.world".to_string());
        let _ = invalid_mail.get_provider();
        let empty_mail_string = Email("".to_string());
        let _ = empty_mail_string.get_provider();
    }

    #[test]
    fn test_get_provider() {
        let email = Email("hello.world@gmail.de".to_string());
        let provider = email.get_provider();
        assert_eq!("gmail.de", provider);
    }

    #[test]
    fn test_get_user() {
        let email = Email("hello.world@gmail.de".to_string());
        let user = email.get_user();
        assert_eq!("hello.world", user);
    }
}