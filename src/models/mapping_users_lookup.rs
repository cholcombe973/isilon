

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingUsersLookup {
  /// Lookup a user access token.
  #[serde(rename = "mapping")]
  mapping: Option<Vec<::models::MappingUsersLookupMappingItem>>
}

