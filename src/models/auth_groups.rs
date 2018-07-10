#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthGroups {
    #[serde(rename = "groups")]
    pub groups: Option<Vec<::models::AuthGroupExtended>>,
}
