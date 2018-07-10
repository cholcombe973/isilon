

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthAccess {
  #[serde(rename = "access")]
  access: Option<Vec<::models::AuthAccessAccessItem>>
}

