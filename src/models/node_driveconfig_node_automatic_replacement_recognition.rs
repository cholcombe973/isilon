

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeDriveconfigNodeAutomaticReplacementRecognition {
  /// Enable automatic replacement recognition (ARR).
  #[serde(rename = "enabled")]
  enabled: Option<bool>
}

