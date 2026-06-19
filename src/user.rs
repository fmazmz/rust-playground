use core::prelude::v1::derive;

use serde::{Deserialize, Serialize};

pub struct User {
    name: String,
    email: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

impl User {
    pub fn new(name: String, email: String) -> Self {
        Self { name, email }
    }

    pub fn greet(&self) {
        println!("Hello {}", self.name);
    }

    // setters
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }

    // getters
    pub fn get_name(&self) -> &str {
        return &self.name;
    }

    pub fn get_email(&self) -> &str {
        return &self.email;
    }
}
