## Basic usage:

Table:

```rs
// Greeting Table
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GreetingTable {
    pub greeting: String,
}

impl GreetingTable {
    pub fn new() -> Self {
        Self {
            greeting: String::from("Hi"),
        }
    }

    pub fn set_greeting(&mut self, greeting: String) {
        self.greeting = greeting;
    }
}
```

App using the Chain DB Client:

```rs
#[tokio::main]
async fn main() {
  // 1 - DB connection
  let db = ChainDB::connect("my-db", "root", "1234");

  // 2 - Init a table
  // Greeting table: table_name | model_instance (schema)
  let mut greeting = db.get_table("greeting", GreetingTable::new).await;
  println!("Current greeting: {:?}", greeting.table.greeting); // Hi

  // 3 - Mutate the table values and persist on chain
  // greeting.table.greeting = "Hello!";
  greeting.table.set_greeting(String::from("Hello!"));
  greeting.persist().await; // Persist data on chain

  // 4 - See the most updated values of the table
  println!("Current greeting: {:?}", greeting.table.greeting); // Hello!
}
```
