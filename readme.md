# Chain DB - RS

ChainDB RS is a library that allows the usage of the ChainDB database in Rust projects.

Chain DB is a Story-driven database. This new type of system uses some features used in blockchain technology. Each change generates a transaction that is saved in a block. The network works centrally, so persistent data is not decentralized.

This database has some features by default, such as: create user account, get user account, transfer units and get transfer records as well as the main feature that is tables.

The `unit` property present in each user's account can be anything the project wants, it can be a type of currency, item, resource.

Visit the [Chain DB repository](https://github.com/wpdas/chain-db) to get to know more.

## Install

Install using cargo. You'll need to install serde json to create your tables structs as well:

```sh
# TODO: Not available yet
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

Tables must a struct. This struct will be used as a reference to create the table's fields.

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

The next examples will not include the `db` implementation and the `async fn main() {}` block as this is implied.

### Get Table's History

You can use `Table.get_history(depth: u64)` to get the last X changes.

```rs
let mut test_table = _db.get_table("test", TestTable::new).await;

// Persist some data
test_table.table.greeting = "Ola amigo!".to_string();
test_table.table.year = 1990;
test_table.persist().await;

test_table.table.greeting = "Hello my dear friend!".to_string();
test_table.table.year = 2012;
test_table.persist().await;

let history = test_table.get_history(50).await;

println!("{:?}", history);
// [
//     TestTable { greeting: 'Hello my dear friend!', year: 2012 }
//     TestTable { greeting: 'Ola amigo!', year: 1990 }
//     TestTable { greeting: 'Oi', year: 2022 }
//     TestTable { greeting: 'E ae!!!', year: 1990 }
//     TestTable { greeting: 'Hi', year: 1999 }
//     ...
// ]
```

This can be useful when the application needs to fetch a list of things, such as messages.

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

### Get User Account Info

This feature can be used for the "Login/Sign In" action.

```rs
let user_name = "wenderson.fake";
let user_pass = "1234";

let user = db.get_user_account(&user_name, user_pass).await;
println!("{:?}", user.data.unwrap());
// SignedUserAccount {
//     id: "b2e4e7c15f733d8c18836ffd22051ed855226d9041fb9452f17f498fc2bcbce3",
//     user_name: "wenderson.fake",
//     units: 2
// }
```

### Get User Account Info By User Id

Just another way to fetch the user info.

```rs
let wenderson_id = "b2e4e7c15f733d8c18836ffd22051ed855226d9041fb9452f17f498fc2bcbce3";
let user = db.get_user_account_by_id(&wenderson_id).await;

println!("{:?}", user.data.unwrap());
// SignedUserAccount {
//     id: "b2e4e7c15f733d8c18836ffd22051ed855226d9041fb9452f17f498fc2bcbce3",
//     user_name: "wenderson.fake",
//     units: 2
// }
```

### Transfer Units Between Two Users

As said before, `unit` property present in each user's account can be anything the project wants, it can be a type of currency, item, resource.

Below is an example of user `wenderson` trying to send 2 units to `suly`:

```rs
let wenderson_id = "b2e4e7c15f733d8c18836ffd22051ed855226d9041fb9452f17f498fc2bcbce3";
let suly_id = "136c406933d98e5c8bb4820f5145869bb5ad40647b768de4e9adb2a52d0dea2f";

let wenderson_data_opt = db.get_user_account_by_id(&wenderson_id).await;
let wenderson_data = wenderson_data_opt.data.unwrap();
let units_to_transfer = 2;

if wenderson_data.units >= units_to_transfer {
    let res = db.transfer_units(&wenderson_id, &suly_id, units_to_transfer).await;
    println!("{:?}", res.success);
    // true / false
}
```

### Fetching the Latest Units Transfer Record

Use this feature to get the last unit transfer record involving a user.

```rs
let wenderson_id = "b2e4e7c15f733d8c18836ffd22051ed855226d9041fb9452f17f498fc2bcbce3";
let last_units_transference_record = db.get_transfer_by_user_id(&wenderson_id).await;

println!("{:?}", last_units_transference_record.data.unwrap());
// TransferUnitsRegistry {
//     from: "b2e4e7c15f733d8c18836ffd22051ed855226d9041fb9452f17f498fc2bcbce3",
//     to: "136c406933d98e5c8bb4820f5145869bb5ad40647b768de4e9adb2a52d0dea2f",
//     units: 2
// }
```

### Fetching All the Transfer of Units Records

Use this feature to get the last unit transfer record involving a user.

```rs
let wenderson_id = "b2e4e7c15f733d8c18836ffd22051ed855226d9041fb9452f17f498fc2bcbce3";
let all_units_transfers_record = db.get_all_transfers_by_user_id(&wenderson_id).await;

println!("{:?}", all_units_transfers_record.data.unwrap());
// [
//    TransferUnitsRegistry {
//        from: "b2e4e7c15f733d8c18836ffd22051ed855226d9041fb9452f17f498fc2bcbce3",
//        to: "136c406933d98e5c8bb4820f5145869bb5ad40647b768de4e9adb2a52d0dea2f",
//        units: 2
//    },
//    TransferUnitsRegistry {
//        from: "b2e4e7c15f733d8c18836ffd22051ed855226d9041fb9452f17f498fc2bcbce3",
//        to: "136c406933d98e5c8bb4820f5145869bb5ad40647b768de4e9adb2a52d0dea2f",
//        units: 2
//    }
// ]
```
