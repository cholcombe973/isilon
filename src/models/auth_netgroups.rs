#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthNetgroups {
    #[serde(rename = "netgroups")]
    pub netgroups: Option<Vec<::models::AuthNetgroup>>,
}
