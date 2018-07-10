

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobPolicyCreateParams {
  /// A helpful human-readable description of the impact policy.
  #[serde(rename = "description")]
  description: Option<String>,
  #[serde(rename = "intervals")]
  intervals: Option<Vec<::models::JobPolicyInterval>>,
  /// The name of the impact policy.
  #[serde(rename = "name")]
  name: String
}

