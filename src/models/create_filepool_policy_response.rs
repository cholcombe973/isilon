#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFilepoolPolicyResponse {
    /// The name of the new policy
    #[serde(rename = "id")]
    pub id: String,
}
