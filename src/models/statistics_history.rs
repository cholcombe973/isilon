

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsHistory {
  #[serde(rename = "stats")]
  stats: Option<Vec<::models::StatisticsHistoryStat>>
}

