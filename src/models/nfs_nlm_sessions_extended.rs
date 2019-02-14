#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NfsNlmSessionsExtended {
    #[serde(rename = "clients")]
    pub clients: Option<Vec <crate::models::NfsNlmSessionsSession>>,
    /// Total number of items available.
    #[serde(rename = "total")]
    pub total: Option<i32>,
}
