#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SmbSettingsShareExtended {
    /// Only enumerate files and folders the requesting user has access to.
    #[serde(rename = "access_based_enumeration")]
    pub access_based_enumeration: Option<bool>,
    /// Access-based enumeration on only the root directory of the share.
    #[serde(rename = "access_based_enumeration_root_only")]
    pub access_based_enumeration_root_only: Option<bool>,
    /// Allow deletion of read-only files in the share.
    #[serde(rename = "allow_delete_readonly")]
    pub allow_delete_readonly: Option<bool>,
    /// Allows users to execute files they have read rights for.
    #[serde(rename = "allow_execute_always")]
    pub allow_execute_always: Option<bool>,
    /// Persistent open timeout for the share.
    #[serde(rename = "ca_timeout")]
    pub ca_timeout: Option<i32>,
    /// Specify the level of write-integrity on continuously available shares.
    #[serde(rename = "ca_write_integrity")]
    pub ca_write_integrity: Option<String>,
    /// Specify level of change notification alerts on the share.
    #[serde(rename = "change_notify")]
    pub change_notify: Option<String>,
    /// Set the create permissions for new files and directories in share.
    #[serde(rename = "create_permissions")]
    pub create_permissions: Option<String>,
    /// Client-side caching policy for the shares.
    #[serde(rename = "csc_policy")]
    pub csc_policy: Option<String>,
    /// Unix umask or mode bits.
    #[serde(rename = "directory_create_mask")]
    pub directory_create_mask: Option<i32>,
    /// Unix umask or mode bits.
    #[serde(rename = "directory_create_mode")]
    pub directory_create_mode: Option<i32>,
    /// Unix umask or mode bits.
    #[serde(rename = "file_create_mask")]
    pub file_create_mask: Option<i32>,
    /// Unix umask or mode bits.
    #[serde(rename = "file_create_mode")]
    pub file_create_mode: Option<i32>,
    /// Specifies the list of file extensions.
    #[serde(rename = "file_filter_extensions")]
    pub file_filter_extensions: Option<Vec<String>>,
    /// Specifies if filter list is for deny or allow. Default is deny.
    #[serde(rename = "file_filter_type")]
    pub file_filter_type: Option<String>,
    /// Enables file filtering on the share.
    #[serde(rename = "file_filtering_enabled")]
    pub file_filtering_enabled: Option<bool>,
    /// Hide files and directories that begin with a period '.'.
    #[serde(rename = "hide_dot_files")]
    pub hide_dot_files: Option<bool>,
    /// An ACL expressing which hosts are allowed access. A deny clause must be the final entry.
    #[serde(rename = "host_acl")]
    pub host_acl: Option<Vec<String>>,
    /// Specify the condition in which user access is done as the guest account.
    #[serde(rename = "impersonate_guest")]
    pub impersonate_guest: Option<String>,
    /// User account to be used as guest account.
    #[serde(rename = "impersonate_user")]
    pub impersonate_user: Option<String>,
    /// Specifies the wchar_t starting point for automatic byte mangling.
    #[serde(rename = "mangle_byte_start")]
    pub mangle_byte_start: Option<i32>,
    /// Character mangle map.
    #[serde(rename = "mangle_map")]
    pub mangle_map: Option<Vec<String>>,
    /// Support NTFS ACLs on files and directories.
    #[serde(rename = "ntfs_acl_support")]
    pub ntfs_acl_support: Option<bool>,
    /// Allow oplock requests.
    #[serde(rename = "oplocks")]
    pub oplocks: Option<bool>,
    /// Specifies if persistent opens would do strict lockout on the share.
    #[serde(rename = "strict_ca_lockout")]
    pub strict_ca_lockout: Option<bool>,
    /// Handle SMB flush operations.
    #[serde(rename = "strict_flush")]
    pub strict_flush: Option<bool>,
    /// Specifies whether byte range locks contend against SMB I/O.
    #[serde(rename = "strict_locking")]
    pub strict_locking: Option<bool>,
    /// Name of the access zone in which to update settings
    #[serde(rename = "zone")]
    pub zone: Option<String>,
}
