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
pub struct GetData {
        name: String,
        age: u32,
        username: String,
}

#[derive(Deserialize)]
pub struct mailerState {
    pub name: String,
    pub email: String,
    pub message: String,
}

impl fmt::Debug for mailerState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("mailerState")
            .field("name", &self.name)
            .field("email", &self.email)
            .field("message", &self.message)
            .finish()
    }
}

impl fmt::Debug for GetData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GetData")
            .field("name", &self.name)
            .field("age", &self.age)
            .field("username", &self.username)
            .finish()
    }
}
