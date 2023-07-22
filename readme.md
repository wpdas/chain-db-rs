# Chain DB - RS

ChainDB RS is a library that allows the usage of the ChainDB database in Rust projects.

Chain DB is a Story-driven database. This new type of system uses some features used in blockchain technology. Each change generates a transaction that is saved in a block. The network works centrally, so persistent data is not decentralized.

This database has some features by default, such as: create user account, get user account, transfer units and get transfer records as well as the main feature that is tables.

The `unit` property present in each user's account can be anything the project wants, it can be a type of currency, item, resource.

Visit the [Chain DB repository](https://github.com/wpdas/chain-db) to get to know more.

## Install

Install using cargo. You'll need to install serde json to create your tables structs as well:

```sh
cargo add chain_db_rs
cargo add serde_json
```

## Usage examples

First of all, it's good to know that all requests return a `BasicResponse<D>` structure that has the following structure:

**success:** `bool` (informs if the transaction was successful) <br/>
**error_msg:** `String` (error message) <br/>
**data:** `D` (any expected data type depending on the request) <br/>

Make sure you have the database running on your local machine or use the server link from where the database is running.

### Table

Tables must be simple class with default values, empty or not. This class will be used as a reference to create the table's fields.

When it's time to persist the table's data on the chain, just call the `persit` database function.

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
}
```

```rs
#[tokio::main]
async fn main() {
  // server | db-name | user | password
  // If the `server` parameter is empty(None), then "http://localhost:2818" will be used.
  let db = ChainDB::connect(None, "my-db", "root", "1234");

  // Initialize the "greeting" table using the "GreetingTable"
  // class as a template. If there is already any data saved in
  // the chain, this data will be populated in the table instance.
  let mut greeting = db.get_table("greeting", GreetingTable::new).await;
  println!("Current greeting: {:?}", greeting.table); // { greeting: 'Hi' }

  // Mutating data
  greeting.table.set_greeting(String::from("Hello my dear!"));
  greeting.persist().await; // Data is persisted on the blockchain

  // See the most updated values of the table
  println!("Current greeting: {:?}", greeting.table); // { greeting: 'Hello my dear!' }
}
```

The next examples will not include the `db` implementation as this is implied.

### Create User Account

This is a default database feature that allows you to create user accounts within the database. As these are hashed accounts, the only data required is: Username and Password. This data is hashed, that is, only the user with the correct data can access the data.

It is not possible to recover an account in which the user has forgotten access data.

```rs
let user_name = "wenderson.fake";
let user_pass = "1234";

// Check if the given name is already in use
let user_name_taken = db.check_user_name(&user_name).await;
if !user_name_taken.success {

    // user name | password | units (optional) | password hint (optional - may be used in the future versions)
    let user = db
        .create_user_account(user_name, user_pass, Some(2), None)
        .await;

    println!("{:?}", user.data.unwrap());
    // SignedUserAccount {
    //     id: "b2e4e7c15f733d8c18836ffd22051ed855226d9041fb9452f17f498fc2bcbce3",
    //     user_name: "wenderson.fake",
    //     units: 2
    // }
}
```
