/// AuthCacheItem : Flush security objects cache.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthCacheItem {
    /// Flush all objects in cache for access zone.
    #[serde(rename = "all")]
    pub all: Option<String>,
    /// Flush objects in cache for access zone specified by GID.
    #[serde(rename = "gid")]
    pub gid: Option<i32>,
    /// Flush objects in cache for access zone specified by group name.
    #[serde(rename = "group_name")]
    pub group_name: Option<String>,
    /// Flush objects in cache for access zone specified by authentication provider.
    #[serde(rename = "provider")]
    pub provider: Option<String>,
    /// Flush objects in cache for access zone specified by SID.
    #[serde(rename = "sid")]
    pub sid: Option<String>,
    /// Flush objects in cache for access zone specified by UID.
    #[serde(rename = "uid")]
    pub uid: Option<i32>,
    /// Flush objects in cache for access zone specified by user name.
    #[serde(rename = "user_name")]
    pub user_name: Option<String>,
}
