

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsHistoryStat {
  /// Devid of node of statistic or 0 for cluster scoped statistics.
  #[serde(rename = "devid")]
  devid: i32,
  /// Key specific error string, if applicable.
  #[serde(rename = "error")]
  error: Option<String>,
  /// Key specific error number, if applicable.
  #[serde(rename = "error_code")]
  error_code: Option<i32>,
  /// Key name of statistic.
  #[serde(rename = "key")]
  key: String,
  /// The interval for which these results were figured (averaged against.)
  #[serde(rename = "resolution")]
  resolution: i32,
  /// Time-series values.
  #[serde(rename = "values")]
  values: Option<Vec<::models::StatisticsHistoryStatValue>>
}

