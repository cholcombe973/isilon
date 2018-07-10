

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NfsAliasExtended {
  /// Specifies whether the alias is usable.
  #[serde(rename = "health")]
  health: Option<String>,
  /// Specifies the name by which the alias can be referenced.
  #[serde(rename = "name")]
  name: Option<String>,
  /// Specifies the path to which the alias points.
  #[serde(rename = "path")]
  path: Option<String>,
  /// Specifies the zone in which the alias is valid.
  #[serde(rename = "zone")]
  zone: Option<String>,
  /// Specifies a string which represents the unique location of the alias.
  #[serde(rename = "id")]
  id: Option<String>
}

