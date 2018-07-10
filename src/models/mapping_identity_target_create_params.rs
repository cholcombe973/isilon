

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingIdentityTargetCreateParams {
  /// Identity is preferred on-disk.
  #[serde(rename = "on_disk")]
  on_disk: Option<bool>,
  /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
  #[serde(rename = "target")]
  target: ::models::AuthAccessAccessItemFileGroup,
  /// Origin of identity mapping.
  #[serde(rename = "type")]
  _type: Option<String>
}

