#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsGlobalGlobalSettings {
    /// Specifies the number of times to retry an ID allocation before failing.
    #[serde(rename = "alloc_retries")]
    pub alloc_retries: Option<i32>,
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
    /// Specifies the user iD for a group when requested by the kernel.
    #[serde(rename = "group_uid")]
    pub group_uid: Option<i32>,
    /// Specifies which providers are loaded by the authentication daemon (lsassd).
    #[serde(rename = "load_providers")]
    pub load_providers: Option<Vec<String>>,
    /// Starts the RID in the local domain to map a UID and a GID.
    #[serde(rename = "min_mapped_rid")]
    pub min_mapped_rid: Option<i32>,
    /// Specifies an alternative GID when the kernel is unable to retrieve a GID for a persona.
    #[serde(rename = "null_gid")]
    pub null_gid: Option<i32>,
    /// Specifies an alternative UID when the kernel is unable to retrieve a UID for a persona.
    #[serde(rename = "null_uid")]
    pub null_uid: Option<i32>,
    /// Specifies the type of identity that is stored on disk.
    #[serde(rename = "on_disk_identity")]
    pub on_disk_identity: Option<String>,
    /// Specifies the minimum amount of time in milliseconds to wait before performing an oprestart.
    #[serde(rename = "rpc_block_time")]
    pub rpc_block_time: Option<i32>,
    /// Specifies the maximum number of outstanding RPC requests.
    #[serde(rename = "rpc_max_requests")]
    pub rpc_max_requests: Option<i32>,
    /// Specifies the maximum amount of time in seconds to wait for an idmap response.
    #[serde(rename = "rpc_timeout")]
    pub rpc_timeout: Option<i32>,
    /// If true, sends NTLMv2 responses.
    #[serde(rename = "send_ntlmv2")]
    pub send_ntlmv2: Option<bool>,
    /// Specifies the space replacement character.
    #[serde(rename = "space_replacement")]
    pub space_replacement: Option<String>,
    /// Specifies the minimum GID to attempt to look up in the idmap database.
    #[serde(rename = "system_gid_threshold")]
    pub system_gid_threshold: Option<i32>,
    /// Specifies the minimum UID to attempt to look up in the idmap database.
    #[serde(rename = "system_uid_threshold")]
    pub system_uid_threshold: Option<i32>,
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
    /// Specifies the GID for the unknown (anonymous) group.
    #[serde(rename = "unknown_gid")]
    pub unknown_gid: Option<i32>,
    /// Specifies the UID for the unknown (anonymous) user.
    #[serde(rename = "unknown_uid")]
    pub unknown_uid: Option<i32>,
    /// Specifies the maximum size (in bytes) of the security object cache in the authentication daemon.
    #[serde(rename = "user_object_cache_size")]
    pub user_object_cache_size: Option<i32>,
    /// Specifies the NetBIOS workgroup or domain.
    #[serde(rename = "workgroup")]
    pub workgroup: Option<String>,
}
