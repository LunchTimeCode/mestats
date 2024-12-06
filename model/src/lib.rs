use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User{
    name: String,
}

impl User {
    pub fn get_name(&self) -> &str {
        &self.name
    }
}