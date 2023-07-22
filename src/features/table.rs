use reqwest::header::CONTENT_TYPE;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::{
    features::{
        constants::{CONTRACT_PAYLOAD, CONTRACT_TRANSACTION},
        structures::TransactionType,
    },
    ChainDB,
};

use super::structures::ContractTransactionData;

#[derive(Debug)]
pub struct Table<Model> {
    pub table: Model,
    contract_id: String,
    db: ChainDB,
}

impl<Model: DeserializeOwned + Serialize> Table<Model> {
    pub async fn get(
        db: &ChainDB,
        table_name: &'static str,
        get_model_instance: fn() -> Model,
    ) -> Table<Model> {
        let contract_id = db.access.parse(db.name.to_string(), table_name.to_string());

        // Load content from chain
        let url = format!(
            "{api}/{path}/{contract_id}/{db_access_key}",
            api = db.api,
            path = CONTRACT_PAYLOAD,
            contract_id = contract_id,
            db_access_key = db.access_key,
        );
        let res_json = reqwest::get(url).await.unwrap().text().await.unwrap();

        // Check if any info was found
        let data_json_check: HashMap<String, Value> = serde_json::from_str(&res_json).unwrap();
        let tx_type_check = data_json_check.get("tx_type").unwrap().as_str().unwrap();
        if tx_type_check == "NONE" {
            return Self {
                contract_id: contract_id.clone(),
                table: get_model_instance(),
                db: db.clone(),
            }
        }

        let contract_data_json: ContractTransactionData<Model> = serde_json::from_str(&res_json).unwrap();

        // If there's already a table (contract) with data, then, fetch its data
        if contract_data_json.tx_type == TransactionType::CONTRACT {
            return Self {
                contract_id: contract_id.clone(),
                table: contract_data_json.data,
                db: db.clone(),
            };
        }

        // If there's no content for this table (contract), then, create a new table
        Self {
            contract_id: contract_id.clone(),
            table: get_model_instance(),
            db: db.clone(),
        }
    }

    /**
     * Persist table data on chain
     */
    pub async fn persist(&self) {
        let client = reqwest::Client::new();
        let url = format!("{api}{route}", api = self.db.api, route = CONTRACT_TRANSACTION);

        let contract_data = serde_json::to_string(&self.table).unwrap();

        let body = json!({
            "tx_type": TransactionType::CONTRACT,
            "contract_id": &self.contract_id,
            "db_access_key": &self.db.access_key,
            "data": &contract_data
        });

        let json_body = serde_json::to_string(&body).unwrap();

        let _ = client
            .post(url)
            .header(CONTENT_TYPE, "application/json")
            .body(json_body)
            .send()
            .await;
    }
}
