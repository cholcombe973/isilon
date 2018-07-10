

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobPolicyExtended {
  /// A helpful human-readable description of the impact policy.
  #[serde(rename = "description")]
  description: String,
  #[serde(rename = "intervals")]
  intervals: Vec<::models::JobPolicyInterval>,
  /// The ID of the impact policy.
  #[serde(rename = "id")]
  id: String,
  /// The name of the impact policy.
  #[serde(rename = "name")]
  name: String,
  /// Whether or not this is a read-only system impact policy.
  #[serde(rename = "system")]
  system: Option<bool>
}

