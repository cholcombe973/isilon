

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthAccessAccessItemShareSharePermissionsShareRelevantAce {
  /// Specifies properties for an Access Control Entry
  #[serde(rename = "ace")]
  ace: Option<String>
}

