

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportAbout {
  #[serde(rename = "reports")]
  reports: Option<Vec<::models::ReportAboutReport>>
}

