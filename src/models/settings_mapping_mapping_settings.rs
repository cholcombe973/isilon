#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsMappingMappingSettings {
    /// Specifies the cache expiry in seconds of the idmapper.
    #[serde(rename = "cache_entry_expiry")]
    pub cache_entry_expiry: Option<i32>,
    /// If true, allocates GIDs from a fixed range.
    #[serde(rename = "gid_range_enabled")]
    pub gid_range_enabled: Option<bool>,
    /// Specifies the ending number for a fixed range from which GIDs are allocated.
    #[serde(rename = "gid_range_max")]
    pub gid_range_max: Option<i32>,
    /// Specifies the starting number for a fixed range from which GIDs are allocated.
    #[serde(rename = "gid_range_min")]
    pub gid_range_min: Option<i32>,
    /// Specifies the next GID to allocate.
    #[serde(rename = "gid_range_next")]
    pub gid_range_next: Option<i32>,
    /// If true, allocates UIDs from a fixed range.
    #[serde(rename = "uid_range_enabled")]
    pub uid_range_enabled: Option<bool>,
    /// Specifies the ending number for a fixed range from which UIDs are allocated.
    #[serde(rename = "uid_range_max")]
    pub uid_range_max: Option<i32>,
    /// Specifies the starting number for a fixed range from which UIDs are allocated.
    #[serde(rename = "uid_range_min")]
    pub uid_range_min: Option<i32>,
    /// Specifies the next UID to allocate.
    #[serde(rename = "uid_range_next")]
    pub uid_range_next: Option<i32>,
}
