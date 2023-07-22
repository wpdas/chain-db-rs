use features::{
    constants::{
        API, CREATE_USER_ACCOUNT, GET_ALL_TRANSFER_BY_USER_ID, GET_TRANSFER_BY_USER_ID,
        GET_USER_ACCOUNT, GET_USER_ACCOUNT_BY_ID, TRANSFER_UNITS, CHECK_USER_NAME,
    },
    structures::{Access, BasicResponse, SignedUserAccount, TransferUnitsRegistry},
    table::Table,
};
use reqwest::header::CONTENT_TYPE;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;

mod features;
mod test;

// ChainDB features
#[derive(Clone, Debug)]
pub struct ChainDB {
    pub api: &'static str,
    pub name: String,
    pub access: Access,
    pub access_key: String,
}

impl ChainDB {
    /**
     * Connection information.
     * If the `server` parameter is empty, then "http://localhost:2818" will be used.
     */
    pub fn connect(server:Option<&'static str>, data_base: &'static str, user: &'static str, password: &'static str) -> Self {
        let key_data = format!(
            "{db_name}{db_user}{db_pass}",
            db_name = data_base,
            db_user = user,
            db_pass = password
        );
        let key = sha256::digest(key_data);

        Self {
            api: server.unwrap_or(API),
            name: data_base.to_string(),
            access: Access {
                user: user,
                password: password,
            },
            // DB Access Key (used to encrypt its data)
            access_key: key,
        }
    }

    /**
     * Create a new user account inside the connected table
     */
    pub async fn create_user_account(
        &self,
        user_name: &str,
        password: &str,
        units: Option<u64>,
        password_hint: Option<String>,
    ) -> BasicResponse<SignedUserAccount> {
        let body = json!({
            "db_access_key": self.access_key,
            "user_name": user_name,
            "password": password,
            "password_hint": password_hint,
            "units": units,
        });

        let json_body = serde_json::to_string(&body).unwrap();

        let url = format!("{api}{route}", api = self.api, route = CREATE_USER_ACCOUNT);

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .header(CONTENT_TYPE, "application/json")
            .body(json_body)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        serde_json::from_str::<BasicResponse<SignedUserAccount>>(&response).unwrap()
    }

    /**
     * Get user account info (login method)
     */
    pub async fn get_user_account(
        &self,
        user_name: &str,
        password: &str,
    ) -> BasicResponse<SignedUserAccount> {
        let url = format!(
            "{api}{route}/{user_name}/{user_pass}/{db_access_key}",
            api = self.api,
            route = GET_USER_ACCOUNT,
            user_name = user_name,
            user_pass = password,
            db_access_key = self.access_key
        );

        let res_json = reqwest::get(url).await.unwrap().text().await.unwrap();

        serde_json::from_str::<BasicResponse<SignedUserAccount>>(&res_json).unwrap()
    }

    /**
     * Get user account info by its id
     */
    pub async fn get_user_account_by_id(
        &self,
        user_id: &str,
    ) -> BasicResponse<SignedUserAccount> {
        let url = format!(
            "{api}{route}/{user_id}/{db_access_key}",
            api = self.api,
            route = GET_USER_ACCOUNT_BY_ID,
            user_id = user_id,
            db_access_key = self.access_key
        );

        let res_json = reqwest::get(url).await.unwrap().text().await.unwrap();

        serde_json::from_str::<BasicResponse<SignedUserAccount>>(&res_json).unwrap()
    }

    /**
     * Check if user_name is already taken
     */
    pub async fn check_user_name(&self, user_name: &str) -> BasicResponse<String> {
        let url = format!(
            "{api}{route}/{user_name}/{db_access_key}",
            api = self.api,
            route = CHECK_USER_NAME,
            user_name = user_name,
            db_access_key = self.access_key
        );

        let response = reqwest::get(url).await.expect("Something went wrong!").text().await.unwrap();

        serde_json::from_str::<BasicResponse<String>>(&response).unwrap()
    }

    /**
     * Transfer units between users
     */
    pub async fn transfer_units(
        &self,
        from: &str,
        to: &str,
        units: u64,
    ) -> BasicResponse<String> {
        let url = format!("{api}{route}", api = self.api, route = TRANSFER_UNITS);

        let body = json!({
            "db_access_key": self.access_key,
            "from": from,
            "to": to,
            "units": units,
        });

        let json_body = serde_json::to_string(&body).unwrap();

        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .header(CONTENT_TYPE, "application/json")
            .body(json_body)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        serde_json::from_str::<BasicResponse<String>>(&response).unwrap()
    }

    /**
     * Fetch the last Transference of units Records by User
     */
    pub async fn get_transfer_by_user_id(
        &self,
        user_id: &str,
    ) -> BasicResponse<TransferUnitsRegistry> {
        let url = format!(
            "{api}{route}/{user_id}/{db_access_key}",
            api = self.api,
            route = GET_TRANSFER_BY_USER_ID,
            user_id = user_id,
            db_access_key = self.access_key,
        );

        let res_json = reqwest::get(url).await.unwrap().text().await.unwrap();

        serde_json::from_str::<BasicResponse<TransferUnitsRegistry>>(&res_json).unwrap()
    }

    /**
     * Fetch all Transference of units Records by User
     */
    pub async fn get_all_transfers_by_user_id(
        &self,
        user_id: &str,
    ) -> BasicResponse<Vec<TransferUnitsRegistry>> {
        let url = format!(
            "{api}{route}/{user_id}/{db_access_key}",
            api = self.api,
            route = GET_ALL_TRANSFER_BY_USER_ID,
            user_id = user_id,
            db_access_key = self.access_key,
        );

        let res_json = reqwest::get(url).await.unwrap().text().await.unwrap();

        serde_json::from_str::<BasicResponse<Vec<TransferUnitsRegistry>>>(&res_json).unwrap()
    }

    /**
     * Initialize a table, fetching its more updated data
     */
    pub async fn get_table<Model: DeserializeOwned + Serialize>(
        &self,
        table_name: &'static str,
        get_model_instance: fn() -> Model,
    ) -> Table<Model> {
        Table::get(&self, table_name, get_model_instance).await
    }
}

#[cfg(test)]
mod tests {
    use crate::test::TestTable;

    use super::*;

    // WARNING: Make sure the ChainDB is running.
    fn random_str() -> String {
        let charset = "abcdefghijklmnopqrstuvwxyz";
        random_string::generate(12, charset)
    }

    // #[tokio::test]
    async fn create_user_account() {
        let db = ChainDB::connect(None, "test-db", "root", "1234");
        let random_user_name = random_str();
        let new_user = db
            .create_user_account(random_user_name.as_str(), "fake123pass", Some(10), None)
            .await;

        assert_eq!(new_user.success, true, "Testing account creation");
    }

    // #[tokio::test]
    async fn create_user_account_with_name_already_taken_return_err() {
        let db = ChainDB::connect(None, "test-db", "root", "1234");
        let random_user_name = random_str();
        let new_user = db
            .create_user_account(random_user_name.as_str(), "fake123pass", Some(10), None)
            .await;

        let new_user_2 = db
            .create_user_account(random_user_name.as_str(), "fake123pass", Some(10), None)
            .await;

        assert_eq!(new_user.success, true);
        assert_eq!(new_user_2.success, false);
        assert_eq!(
            new_user_2.error_msg,
            "This user name is already taken".to_string()
        );
    }

    // #[tokio::test]
    async fn get_user_info_with_user_and_password() {
        let db = ChainDB::connect(None, "test-db", "root", "1234");
        let random_user_name = random_str();
        let new_user = db
            .create_user_account(random_user_name.as_str(), "fake123pass", Some(10), None)
            .await;

        let info_user_call = db
            .get_user_account(random_user_name.as_str(), "fake123pass")
            .await;
        let user = info_user_call.data.unwrap();
        assert_eq!(user.user_name, random_user_name.as_str());
        assert_eq!(user.units, 10);
        assert_eq!(user.id, new_user.data.unwrap().id);
    }

    async fn get_user_info_by_id() {
        let db = ChainDB::connect(None, "test-db", "root", "1234");
        let random_user_name = random_str();
        let new_user = db
            .create_user_account(random_user_name.as_str(), "fake123pass", Some(10), None)
            .await;
        let user_id = new_user.data.unwrap().id;

        let info_user_call = db.get_user_account_by_id(&user_id).await;
        let user = info_user_call.data.unwrap();
        assert_eq!(user.user_name, random_user_name.as_str());
        assert_eq!(user.units, 10);
    }

    // #[tokio::test]
    async fn transfer_units_between_two_users() {
        let db = ChainDB::connect(None, "test-db", "root", "1234");
        let random_user_name_1 = random_str();
        let new_user = db
            .create_user_account(random_user_name_1.as_str(), "fake123pass", Some(10), None)
            .await;
        let user_id_1 = new_user.data.unwrap().id;

        let random_user_name_2 = random_str();
        let new_user_2 = db
            .create_user_account(random_user_name_2.as_str(), "fake123pass", None, None)
            .await;
        let user_id_2 = new_user_2.data.unwrap().id;

        let tranference_response = db.transfer_units(&user_id_1, &user_id_2, 6).await;
        assert_eq!(tranference_response.success, true);

        // Ensure users have updated units
        let info_user_1 = db.get_user_account_by_id(&user_id_1).await;
        let info_user_2 = db.get_user_account_by_id(&user_id_2).await;
        assert_eq!(info_user_1.data.unwrap().units, 4);
        assert_eq!(info_user_2.data.unwrap().units, 6);
    }

    async fn transfer_units_between_two_users_with_no_enough_units_err() {
        let db = ChainDB::connect(None, "test-db", "root", "1234");
        let random_user_name_1 = random_str();
        let new_user = db
            .create_user_account(random_user_name_1.as_str(), "fake123pass", Some(10), None)
            .await;
        let user_id_1 = new_user.data.unwrap().id;

        let random_user_name_2 = random_str();
        let new_user_2 = db
            .create_user_account(random_user_name_2.as_str(), "fake123pass", None, None)
            .await;
        let user_id_2 = new_user_2.data.unwrap().id;

        let tranference_response = db.transfer_units(&user_id_1, &user_id_2, 11).await;
        assert_eq!(
            tranference_response.error_msg,
            "Sender user does not have enough units"
        );
    }

    async fn get_user_tranfer_record() {
        let db = ChainDB::connect(None, "test-db", "root", "1234");
        let random_user_name_1 = random_str();
        let new_user = db
            .create_user_account(random_user_name_1.as_str(), "fake123pass", Some(10), None)
            .await;
        let user_id_1 = new_user.data.unwrap().id;

        let random_user_name_2 = random_str();
        let new_user_2 = db
            .create_user_account(random_user_name_2.as_str(), "fake123pass", None, None)
            .await;
        let user_id_2 = new_user_2.data.unwrap().id;

        let tranference_response = db.transfer_units(&user_id_1, &user_id_2, 6).await;
        assert_eq!(tranference_response.success, true);

        let last_transfer_record = db.get_transfer_by_user_id(&user_id_1).await;
        let transfer = last_transfer_record.data.unwrap();

        assert_eq!(transfer.from, user_id_1);
        assert_eq!(transfer.to, user_id_2);
        assert_eq!(transfer.units, 6);
    }

    async fn get_all_user_tranfer_records() {
        let db = ChainDB::connect(None, "test-db", "root", "1234");
        let random_user_name_1 = random_str();
        let new_user = db
            .create_user_account(random_user_name_1.as_str(), "fake123pass", Some(10), None)
            .await;
        let user_id_1 = new_user.data.unwrap().id;

        let random_user_name_2 = random_str();
        let new_user_2 = db
            .create_user_account(random_user_name_2.as_str(), "fake123pass", None, None)
            .await;
        let user_id_2 = new_user_2.data.unwrap().id;

        let _ = db.transfer_units(&user_id_1, &user_id_2, 6).await;
        let _ = db.transfer_units(&user_id_1, &user_id_2, 2).await;

        let last_transfer_records = db.get_all_transfers_by_user_id(&user_id_1).await;
        let transfers = last_transfer_records.data.unwrap();

        assert_eq!(transfers.len(), 2);
    }

    async fn create_table_and_write_read_data() {
        let db = ChainDB::connect(None, "test-db", "root", "1234");
        let mut test_table = db.get_table("test", TestTable::new).await;
        assert_eq!(test_table.table.greeting, String::from("Hi"));
        assert_eq!(test_table.table.year, 2023);
        test_table.table.set_greeting(String::from("Hello"));
        test_table.table.year = 2024;
        test_table.persist().await;
        assert_eq!(test_table.table.greeting, String::from("Hello"));
        assert_eq!(test_table.table.year, 2024);
        // Reset table values
        test_table.table.greeting = String::from("Hi");
        test_table.table.year = 2023;
        test_table.persist().await;
    }

    #[tokio::test]
    async fn integration_all_features() {
        create_user_account().await;
        create_user_account_with_name_already_taken_return_err().await;
        get_user_info_with_user_and_password().await;
        get_user_info_by_id().await;
        transfer_units_between_two_users().await;
        transfer_units_between_two_users_with_no_enough_units_err().await;
        get_user_tranfer_record().await;
        get_all_user_tranfer_records().await;
        create_table_and_write_read_data().await;
    }

}