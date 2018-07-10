

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthIdNtoken {
  /// Specifies additional UIDs, GIDs, and SIDs.
  #[serde(rename = "additional_id")]
  additional_id: Option<Vec<::models::AuthAccessAccessItemFileGroup>>,
  /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
  #[serde(rename = "gid")]
  gid: Option<::models::AuthAccessAccessItemFileGroup>,
  /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
  #[serde(rename = "group_sid")]
  group_sid: Option<::models::AuthAccessAccessItemFileGroup>,
  /// Indicates if this user has restricted access to the /ifs file system.
  #[serde(rename = "ifs_restricted")]
  ifs_restricted: Option<bool>,
  #[serde(rename = "local_address")]
  local_address: Option<String>,
  /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
  #[serde(rename = "on_disk_group_id")]
  on_disk_group_id: Option<::models::AuthAccessAccessItemFileGroup>,
  /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
  #[serde(rename = "on_disk_user_id")]
  on_disk_user_id: Option<::models::AuthAccessAccessItemFileGroup>,
  /// Specifies the privileges granted to the currently authenticated user.
  #[serde(rename = "privilege")]
  privilege: Option<Vec<::models::AuthIdNtokenPrivilegeItem>>,
  #[serde(rename = "protocol")]
  protocol: Option<i32>,
  #[serde(rename = "remote_address")]
  remote_address: Option<String>,
  /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
  #[serde(rename = "uid")]
  uid: Option<::models::AuthAccessAccessItemFileGroup>,
  /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
  #[serde(rename = "user_sid")]
  user_sid: Option<::models::AuthAccessAccessItemFileGroup>,
  #[serde(rename = "zid")]
  zid: Option<i32>,
  #[serde(rename = "zone_id")]
  zone_id: Option<String>
}

