#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingUsersLookupMappingItemGroup {
    #[serde(rename = "dn")]
    pub dn: Option<String>,
    #[serde(rename = "dns_domain")]
    pub dns_domain: Option<String>,
    #[serde(rename = "domain")]
    pub domain: Option<String>,
    #[serde(rename = "email")]
    pub email: Option<String>,
    /// If true, the authenticated user is enabled.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// If true, the authenticated auth user is expired.
    #[serde(rename = "expired")]
    pub expired: Option<bool>,
    #[serde(rename = "expiry")]
    pub expiry: Option<i32>,
    #[serde(rename = "gecos")]
    pub gecos: Option<String>,
    /// If true, indicates that the GID was generated.
    #[serde(rename = "generated_gid")]
    pub generated_gid: Option<bool>,
    /// If true, indicates that the UID was generated.
    #[serde(rename = "generated_uid")]
    pub generated_uid: Option<bool>,
    /// If true, indicates that the UPN was generated.
    #[serde(rename = "generated_upn")]
    pub generated_upn: Option<bool>,
    /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
    #[serde(rename = "gid")]
    pub gid: Option <crate::models::AuthAccessAccessItemFileGroup>,
    #[serde(rename = "home_directory")]
    pub home_directory: Option<String>,
    /// Specifies the user or group ID.
    #[serde(rename = "id")]
    pub id: String,
    /// If true, the account is locked out.
    #[serde(rename = "locked")]
    pub locked: Option<bool>,
    /// Specifies the maximum time in seconds allowed before the password expires.
    #[serde(rename = "max_password_age")]
    pub max_password_age: Option<i32>,
    #[serde(rename = "member_of")]
    pub member_of: Option<Vec <crate::models::AuthAccessAccessItemFileGroup>>,
    /// Specifies a user or group name.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "object_history")]
    pub object_history: Option<Vec <crate::models::AuthGroupObjectHistoryItem>>,
    /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
    #[serde(rename = "on_disk_group_identity")]
    pub on_disk_group_identity: Option <crate::models::AuthAccessAccessItemFileGroup>,
    /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
    #[serde(rename = "on_disk_user_identity")]
    pub on_disk_user_identity: Option <crate::models::AuthAccessAccessItemFileGroup>,
    /// If true, the password has expired.
    #[serde(rename = "password_expired")]
    pub password_expired: Option<bool>,
    /// If true, the password is allowed to expire.
    #[serde(rename = "password_expires")]
    pub password_expires: Option<bool>,
    #[serde(rename = "password_expiry")]
    pub password_expiry: Option<i32>,
    #[serde(rename = "password_last_set")]
    pub password_last_set: Option<i32>,
    /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
    #[serde(rename = "primary_group_sid")]
    pub primary_group_sid: Option <crate::models::AuthAccessAccessItemFileGroup>,
    /// If true, prompts the user to change their password on next login.
    #[serde(rename = "prompt_password_change")]
    pub prompt_password_change: Option<bool>,
    #[serde(rename = "provider")]
    pub provider: Option<String>,
    #[serde(rename = "sam_account_name")]
    pub sam_account_name: Option<String>,
    #[serde(rename = "shell")]
    pub shell: Option<String>,
    /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
    #[serde(rename = "sid")]
    pub sid: Option <crate::models::AuthAccessAccessItemFileGroup>,
    /// Specifies the object type.
    #[serde(rename = "type")]
    pub _type: String,
    /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
    #[serde(rename = "uid")]
    pub uid: Option <crate::models::AuthAccessAccessItemFileGroup>,
    #[serde(rename = "upn")]
    pub upn: Option<String>,
    /// If true, the user password can be changed.
    #[serde(rename = "user_can_change_password")]
    pub user_can_change_password: Option<bool>,
}
