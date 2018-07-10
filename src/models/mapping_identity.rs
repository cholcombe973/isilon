

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingIdentity {
  /// Specifies the identity mapping entry id.
  #[serde(rename = "id")]
  id: String,
  /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
  #[serde(rename = "source")]
  source: Option<::models::AuthAccessAccessItemFileGroup>,
  #[serde(rename = "targets")]
  targets: Vec<::models::MappingIdentityTarget>
}

