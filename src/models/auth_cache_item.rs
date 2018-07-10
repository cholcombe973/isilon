
/// AuthCacheItem : Flush security objects cache.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthCacheItem {
  /// Flush all objects in cache for access zone.
  #[serde(rename = "all")]
  all: Option<String>,
  /// Flush objects in cache for access zone specified by GID.
  #[serde(rename = "gid")]
  gid: Option<i32>,
  /// Flush objects in cache for access zone specified by group name.
  #[serde(rename = "group_name")]
  group_name: Option<String>,
  /// Flush objects in cache for access zone specified by authentication provider.
  #[serde(rename = "provider")]
  provider: Option<String>,
  /// Flush objects in cache for access zone specified by SID.
  #[serde(rename = "sid")]
  sid: Option<String>,
  /// Flush objects in cache for access zone specified by UID.
  #[serde(rename = "uid")]
  uid: Option<i32>,
  /// Flush objects in cache for access zone specified by user name.
  #[serde(rename = "user_name")]
  user_name: Option<String>
}

