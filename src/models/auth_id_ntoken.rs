#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthIdNtoken {
    /// Specifies additional UIDs, GIDs, and SIDs.
    #[serde(rename = "additional_id")]
    pub additional_id: Option<Vec<::models::AuthAccessAccessItemFileGroup>>,
    /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
    #[serde(rename = "gid")]
    pub gid: Option<::models::AuthAccessAccessItemFileGroup>,
    /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
    #[serde(rename = "group_sid")]
    pub group_sid: Option<::models::AuthAccessAccessItemFileGroup>,
    /// Indicates if this user has restricted access to the /ifs file system.
    #[serde(rename = "ifs_restricted")]
    pub ifs_restricted: Option<bool>,
    #[serde(rename = "local_address")]
    pub local_address: Option<String>,
    /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
    #[serde(rename = "on_disk_group_id")]
    pub on_disk_group_id: Option<::models::AuthAccessAccessItemFileGroup>,
    /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
    #[serde(rename = "on_disk_user_id")]
    pub on_disk_user_id: Option<::models::AuthAccessAccessItemFileGroup>,
    /// Specifies the privileges granted to the currently authenticated user.
    #[serde(rename = "privilege")]
    pub privilege: Option<Vec<::models::AuthIdNtokenPrivilegeItem>>,
    #[serde(rename = "protocol")]
    pub protocol: Option<i32>,
    #[serde(rename = "remote_address")]
    pub remote_address: Option<String>,
    /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
    #[serde(rename = "uid")]
    pub uid: Option<::models::AuthAccessAccessItemFileGroup>,
    /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
    #[serde(rename = "user_sid")]
    pub user_sid: Option<::models::AuthAccessAccessItemFileGroup>,
    #[serde(rename = "zid")]
    pub zid: Option<i32>,
    #[serde(rename = "zone_id")]
    pub zone_id: Option<String>,
}
