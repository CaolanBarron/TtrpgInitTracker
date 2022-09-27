
use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Deserialize, Serialize)]
pub struct Character {
    pub name: String,
    pub initiative: Option<u8>,
}

impl Character {
    pub fn new(name: String, initiative: Option<u8>) -> Self {
        Self {
            name,
            initiative,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn initiative(&self) -> &Option<u8>  {
        &self.initiative
    }
}

impl Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name: {}, Initiative: {}", self.name(), self.initiative().unwrap())
    }
}