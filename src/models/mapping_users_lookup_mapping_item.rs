

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingUsersLookupMappingItem {
  #[serde(rename = "groups")]
  groups: Option<Vec<::models::MappingUsersLookupMappingItemGroup>>,
  #[serde(rename = "object_history")]
  object_history: Option<Vec<::models::AuthGroupObjectHistoryItem>>,
  #[serde(rename = "privileges")]
  privileges: Option<Vec<::models::AuthIdNtokenPrivilegeItem>>,
  /// Specifies the configuration properties for a user.
  #[serde(rename = "user")]
  user: Option<::models::MappingUsersLookupMappingItemUser>,
  #[serde(rename = "zid")]
  zid: Option<i32>,
  #[serde(rename = "zone")]
  zone: Option<String>
}

