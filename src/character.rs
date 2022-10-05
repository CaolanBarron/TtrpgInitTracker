use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize, Ord, PartialOrd, PartialEq, Eq)]
pub struct Character {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub initiative: Option<u8>,
}

impl Character {
    pub(crate) fn new(name: String, initiative: Option<u8>) -> Self {
        Self { name, initiative }
    }

    pub(crate) fn name(&self) -> &String {
        &self.name
    }

    pub(crate) fn initiative(&self) -> &Option<u8> {
        &self.initiative
    }
}

impl Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name: {}, Initiative: {}",
            self.name(),
            self.initiative().unwrap()
        )
    }
}
