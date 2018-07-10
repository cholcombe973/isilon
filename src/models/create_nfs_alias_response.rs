
/// CreateNfsAliasResponse : Specifies the return value when an alias is successfully created or modified.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNfsAliasResponse {
  /// Specifies whether the alias is usable.
  #[serde(rename = "health")]
  health: Option<String>,
  /// Specifies a string which represents the unique location of the alias.
  #[serde(rename = "id")]
  id: String
}

