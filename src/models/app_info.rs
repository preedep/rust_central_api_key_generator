use serde::{Serialize,Deserialize};

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