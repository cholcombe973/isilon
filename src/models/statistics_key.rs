

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsKey {
  /// Type of aggregation used in down-sampling.
  #[serde(rename = "aggregation_type")]
  aggregation_type: String,
  /// Name of key this keys is derived from, if any.
  #[serde(rename = "base_name")]
  base_name: Option<String>,
  /// Default time in seconds system will used cached values.
  #[serde(rename = "default_cache_time")]
  default_cache_time: i32,
  /// Description of statistics key.
  #[serde(rename = "description")]
  description: String,
  /// Key name.
  #[serde(rename = "key")]
  key: String,
  /// List of effective history policies for key.
  #[serde(rename = "policies")]
  policies: Option<Vec<::models::StatisticsKeyPolicy>>,
  /// Configured time in seconds system will used cached values.
  #[serde(rename = "policy_cache_time")]
  policy_cache_time: Option<i32>,
  /// Name of real key if this is an alias.
  #[serde(rename = "real_name")]
  real_name: Option<String>,
  /// Scope of key.
  #[serde(rename = "scope")]
  scope: String,
  /// Data type of key values.
  #[serde(rename = "type")]
  _type: String,
  /// Units of key values.
  #[serde(rename = "units")]
  units: String
}

