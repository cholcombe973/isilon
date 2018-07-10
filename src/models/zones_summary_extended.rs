

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ZonesSummaryExtended {
  /// The summary of a collection of objects.
  #[serde(rename = "summary")]
  summary: Option<::models::ZonesSummarySummaryExtended>
}

