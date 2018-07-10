

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobJobSummary {
  /// 
  #[serde(rename = "summary")]
  summary: Option<::models::JobJobSummarySummary>
}

