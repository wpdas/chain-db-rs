use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TransactionType {
    NONE,
    ACCOUNT,
    CONTRACT,
    TRANSFER,
}

#[derive(Clone)]
pub struct Access {
    pub user: &'static str,
    pub password: &'static str,
}

impl Access {
    /**
     * Create the contract id hash using data_base, contract_id, user and password information
     */
    pub fn parse(&self, data_base: String, table_name: String) -> String {
        let access_info = format!(
            "{data_base}{table_name}{user}{password}",
            data_base = data_base,
            table_name = table_name,
            user = self.user,
            password = self.password
        );
        sha256::digest(access_info)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserAccount {
    pub user_name: String,
    pub units: u64, // coins
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignedUserAccount {
    pub id: String, // Used to refer the user
    pub user_name: String,
    pub units: u64,
}

impl SignedUserAccount {
    pub fn to_user(&self) -> UserAccount {
        UserAccount {
            user_name: self.user_name.clone(),
            units: self.units,
        }
    }
}
