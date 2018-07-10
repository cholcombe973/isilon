#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FsaResultsExtended {
    #[serde(rename = "results")]
    pub results: Option<Vec<::models::FsaResultExtended>>,
    /// Total number of items available.
    #[serde(rename = "total")]
    pub total: Option<i32>,
}
