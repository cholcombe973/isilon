#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncJobs {
    #[serde(rename = "jobs")]
    pub jobs: Option<Vec <crate::models::SyncJobExtended>>,
}
