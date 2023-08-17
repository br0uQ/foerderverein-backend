use serde::Serialize;
use serde::Deserialize;
use core::fmt;

#[derive(Serialize)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub favourite_food: Option<String>,
    pub id: u32
}

#[derive(Deserialize)]
pub struct MailerState {
    pub name: String,
    pub email: String,
    pub message: String,
}

impl fmt::Debug for MailerState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MailerState")
            .field("name", &self.name)
            .field("email", &self.email)
            .field("message", &self.message)
            .finish()
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct User {
    pub id: u64,
    pub username: String,
}

