#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryWorkload {
    #[serde(rename = "workload")]
    pub workload: Option<Vec<::models::SummaryWorkloadWorkloadItem>>,
}
