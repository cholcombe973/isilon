
/// NodeDriveconfigExtended : An object containing a node's drive subsystem XML configuration file.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeDriveconfigExtended {
  /// Configuration settings for automatic replacement recognition (ARR).
  #[serde(rename = "automatic_replacement_recognition")]
  automatic_replacement_recognition: Option<::models::NodeDriveconfigNodeAutomaticReplacementRecognition>
}

