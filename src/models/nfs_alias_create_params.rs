

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NfsAliasCreateParams {
  /// Specifies whether the alias is usable.
  #[serde(rename = "health")]
  health: Option<String>,
  /// Specifies the name by which the alias can be referenced.
  #[serde(rename = "name")]
  name: String,
  /// Specifies the path to which the alias points.
  #[serde(rename = "path")]
  path: String,
  /// Specifies the zone in which the alias is valid.
  #[serde(rename = "zone")]
  zone: Option<String>
}

