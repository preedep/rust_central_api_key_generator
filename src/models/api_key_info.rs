// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize)]
pub struct APIKeyInfo {

    #[serde(rename = "application_id")]
    pub application_id: String,

    #[serde(rename = "environment")]
    pub environment: String,

    #[serde(rename = "api_key_id")]
    pub api_key_id: Option<String>,

    #[serde(rename = "secret_key_id")]
    pub secret_key_id: Option<String>,

    #[serde(rename = "call_back_url")]
    pub call_back_url: Option<String>,

    #[serde(rename = "call_back_url_status")]
    pub call_back_url_status : Option<String>,

    #[serde(rename = "created_dt")]
    pub created_dt: Option<String>,

    #[serde(rename = "updated_dt")]
    pub updated_dt: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AppInfo {

    #[serde(rename = "application_id")]
    pub application_id: String,

    #[serde(rename = "application_name")]
    pub application_name: Option<String>,

    #[serde(rename = "created_dt")]
    pub created_dt: Option<String>,

    #[serde(rename = "updated_dt")]
    pub updated_dt: Option<String>
}