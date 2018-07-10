#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IdResolutionPathsError {
    #[serde(rename = "field")]
    pub field: Option<String>,
    #[serde(rename = "field_value")]
    pub field_value: Option<String>,
    #[serde(rename = "message")]
    pub message: Option<String>,
}
