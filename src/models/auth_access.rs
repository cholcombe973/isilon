#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthAccess {
    #[serde(rename = "access")]
    pub access: Option<Vec <crate::models::AuthAccessAccessItem>>,
}
