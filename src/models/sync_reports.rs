#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncReports {
    #[serde(rename = "reports")]
    pub reports: Option<Vec<::models::SyncReport>>,
}
