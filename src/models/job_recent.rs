#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobRecent {
    #[serde(rename = "recent_jobs")]
    pub recent_jobs: Option<Vec <crate::models::JobJobExtended>>,
    /// Total number of items available.
    #[serde(rename = "total")]
    pub total: Option<i32>,
}
