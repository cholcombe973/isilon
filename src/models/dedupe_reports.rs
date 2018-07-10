

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DedupeReports {
  #[serde(rename = "reports")]
  reports: Option<Vec<::models::DedupeReportExtended>>
}

