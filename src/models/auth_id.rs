#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthId {
    /// Specifies properties for a security token for the currently authenticated user.
    #[serde(rename = "ntoken")]
    pub ntoken: Option <crate::models::AuthIdNtoken>,
}
