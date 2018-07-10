

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobPolicy {
  /// A helpful human-readable description of the impact policy.
  #[serde(rename = "description")]
  description: Option<String>,
  #[serde(rename = "intervals")]
  intervals: Option<Vec<::models::JobPolicyInterval>>
}

