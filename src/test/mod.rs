use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TestTable {
    pub greeting: String,
    pub year: u16,
}

impl TestTable {
    pub fn new() -> Self {
        Self {
            greeting: String::from("Hi"),
            year: 2023,
        }
    }

    pub fn set_greeting(&mut self, greeting: String) {
        self.greeting = greeting;
    }
}
