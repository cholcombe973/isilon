#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryQuery {
    #[serde(rename = "result")]
    pub result: Option<Vec<String>>,
    #[serde(rename = "scope")]
    pub scope: Option<::models::DirectoryQueryScope>,
}
