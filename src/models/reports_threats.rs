

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportsThreats {
  #[serde(rename = "reports")]
  reports: Option<Vec<::models::ReportsThreatsReport>>
}

