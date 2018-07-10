

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryWorkload {
  #[serde(rename = "workload")]
  workload: Option<Vec<::models::SummaryWorkloadWorkloadItem>>
}

