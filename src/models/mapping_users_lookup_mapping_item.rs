#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingUsersLookupMappingItem {
    #[serde(rename = "groups")]
    pub groups: Option<Vec<::models::MappingUsersLookupMappingItemGroup>>,
    #[serde(rename = "object_history")]
    pub object_history: Option<Vec<::models::AuthGroupObjectHistoryItem>>,
    #[serde(rename = "privileges")]
    pub privileges: Option<Vec<::models::AuthIdNtokenPrivilegeItem>>,
    /// Specifies the configuration properties for a user.
    #[serde(rename = "user")]
    pub user: Option<::models::MappingUsersLookupMappingItemUser>,
    #[serde(rename = "zid")]
    pub zid: Option<i32>,
    #[serde(rename = "zone")]
    pub zone: Option<String>,
}
