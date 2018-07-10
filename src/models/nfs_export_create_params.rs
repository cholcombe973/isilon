#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NfsExportCreateParams {
    /// True if all directories under the specified paths are mountable.
    #[serde(rename = "all_dirs")]
    pub all_dirs: Option<bool>,
    /// Specifies the block size returned by the NFS statfs procedure.
    #[serde(rename = "block_size")]
    pub block_size: Option<i32>,
    /// True if the client can set file times through the NFS set attribute request. This parameter does not affect server behavior, but is included to accommoate legacy client requirements.
    #[serde(rename = "can_set_time")]
    pub can_set_time: Option<bool>,
    /// True if the case is ignored for file names. This parameter does not affect server behavior, but is included to accommodate legacy client requirements.
    #[serde(rename = "case_insensitive")]
    pub case_insensitive: Option<bool>,
    /// True if the case is preserved for file names. This parameter does not affect server behavior, but is included to accommodate legacy client requirements.
    #[serde(rename = "case_preserving")]
    pub case_preserving: Option<bool>,
    /// True if the superuser can change file ownership. This parameter does not affect server behavior, but is included to accommodate legacy client requirements.
    #[serde(rename = "chown_restricted")]
    pub chown_restricted: Option<bool>,
    /// Specifies the clients with root access to the export.
    #[serde(rename = "clients")]
    pub clients: Option<Vec<String>>,
    /// True if NFS  commit  requests execute asynchronously.
    #[serde(rename = "commit_asynchronous")]
    pub commit_asynchronous: Option<bool>,
    /// Specifies the user-defined string that is used to identify the export.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Specifies the preferred size for directory read operations. This value is used to advise the client of optimal settings for the server, but is not enforced.
    #[serde(rename = "directory_transfer_size")]
    pub directory_transfer_size: Option<i32>,
    /// Specifies the default character set encoding of the clients connecting to the export, unless otherwise specified.
    #[serde(rename = "encoding")]
    pub encoding: Option<String>,
    /// Specifies the reported maximum number of links to a file. This parameter does not affect server behavior, but is included to accommodate legacy client requirements.
    #[serde(rename = "link_max")]
    pub link_max: Option<i32>,
    /// User and group mapping.
    #[serde(rename = "map_all")]
    pub map_all: Option<::models::NfsSettingsExportSettingsMapAll>,
    /// User and group mapping.
    #[serde(rename = "map_failure")]
    pub map_failure: Option<::models::NfsSettingsExportSettingsMapAll>,
    /// True if user mappings query the OneFS user database. When set to false, user mappings only query local authentication.
    #[serde(rename = "map_full")]
    pub map_full: Option<bool>,
    /// True if incoming user IDs (UIDs) are mapped to users in the OneFS user database. When set to false, incoming UIDs are applied directly to file operations.
    #[serde(rename = "map_lookup_uid")]
    pub map_lookup_uid: Option<bool>,
    /// User and group mapping.
    #[serde(rename = "map_non_root")]
    pub map_non_root: Option<::models::NfsSettingsExportSettingsMapAll>,
    /// Determines whether searches for users specified in 'map_all', 'map_root' or 'map_nonroot' are retried if the search fails.
    #[serde(rename = "map_retry")]
    pub map_retry: Option<bool>,
    /// User and group mapping.
    #[serde(rename = "map_root")]
    pub map_root: Option<::models::NfsSettingsExportSettingsMapAll>,
    /// Specifies the maximum file size for any file accessed from the export. This parameter does not affect server behavior, but is included to accommodate legacy client requirements.
    #[serde(rename = "max_file_size")]
    pub max_file_size: Option<i32>,
    /// Specifies the reported maximum length of a file name. This parameter does not affect server behavior, but is included to accommodate legacy client requirements.
    #[serde(rename = "name_max_size")]
    pub name_max_size: Option<i32>,
    /// True if long file names result in an error. This parameter does not affect server behavior, but is included to accommodate legacy client requirements.
    #[serde(rename = "no_truncate")]
    pub no_truncate: Option<bool>,
    /// Specifies the paths under /ifs that are exported.
    #[serde(rename = "paths")]
    pub paths: Vec<String>,
    /// True if the export is set to read-only.
    #[serde(rename = "read_only")]
    pub read_only: Option<bool>,
    /// Specifies the clients with read-only access to the export.
    #[serde(rename = "read_only_clients")]
    pub read_only_clients: Option<Vec<String>>,
    /// Specifies the maximum buffer size that clients should use on NFS read requests. This value is used to advise the client of optimal settings for the server, but is not enforced.
    #[serde(rename = "read_transfer_max_size")]
    pub read_transfer_max_size: Option<i32>,
    /// Specifies the preferred multiple size for NFS read requests. This value is used to advise the client of optimal settings for the server, but is not enforced.
    #[serde(rename = "read_transfer_multiple")]
    pub read_transfer_multiple: Option<i32>,
    /// Specifies the preferred size for NFS read requests. This value is used to advise the client of optimal settings for the server, but is not enforced.
    #[serde(rename = "read_transfer_size")]
    pub read_transfer_size: Option<i32>,
    /// Specifies the clients with both read and write access to the export, even when the export is set to read-only.
    #[serde(rename = "read_write_clients")]
    pub read_write_clients: Option<Vec<String>>,
    /// True if 'readdirplus' requests are enabled. Enabling this property might improve network performance and is only available for NFSv3.
    #[serde(rename = "readdirplus")]
    pub readdirplus: Option<bool>,
    /// Sets the number of directory entries that are prefetched when a 'readdirplus' request is processed. (Deprecated.)
    #[serde(rename = "readdirplus_prefetch")]
    pub readdirplus_prefetch: Option<i32>,
    /// Limits the size of file identifiers returned by NFSv3+ to 32-bit values (may require remount).
    #[serde(rename = "return_32bit_file_ids")]
    pub return_32bit_file_ids: Option<bool>,
    /// Clients that have root access to the export.
    #[serde(rename = "root_clients")]
    pub root_clients: Option<Vec<String>>,
    /// Specifies the authentication types that are supported for this export.
    #[serde(rename = "security_flavors")]
    pub security_flavors: Option<Vec<String>>,
    /// True if set attribute operations execute asynchronously.
    #[serde(rename = "setattr_asynchronous")]
    pub setattr_asynchronous: Option<bool>,
    /// Specifies the snapshot for all mounts.
    #[serde(rename = "snapshot")]
    pub snapshot: Option<String>,
    /// True if symlinks are supported. This value is used to advise the client of optimal settings for the server, but is not enforced.
    #[serde(rename = "symlinks")]
    pub symlinks: Option<bool>,
    /// Specifies the resolution of all time values that are returned to the clients
    #[serde(rename = "time_delta")]
    pub time_delta: Option<f32>,
    /// Specifies the synchronization type.
    #[serde(rename = "write_datasync_action")]
    pub write_datasync_action: Option<String>,
    /// Specifies the synchronization type.
    #[serde(rename = "write_datasync_reply")]
    pub write_datasync_reply: Option<String>,
    /// Specifies the synchronization type.
    #[serde(rename = "write_filesync_action")]
    pub write_filesync_action: Option<String>,
    /// Specifies the synchronization type.
    #[serde(rename = "write_filesync_reply")]
    pub write_filesync_reply: Option<String>,
    /// Specifies the maximum buffer size that clients should use on NFS write requests. This value is used to advise the client of optimal settings for the server, but is not enforced.
    #[serde(rename = "write_transfer_max_size")]
    pub write_transfer_max_size: Option<i32>,
    /// Specifies the preferred multiple size for NFS write requests. This value is used to advise the client of optimal settings for the server, but is not enforced.
    #[serde(rename = "write_transfer_multiple")]
    pub write_transfer_multiple: Option<i32>,
    /// Specifies the preferred multiple size for NFS write requests. This value is used to advise the client of optimal settings for the server, but is not enforced.
    #[serde(rename = "write_transfer_size")]
    pub write_transfer_size: Option<i32>,
    /// Specifies the synchronization type.
    #[serde(rename = "write_unstable_action")]
    pub write_unstable_action: Option<String>,
    /// Specifies the synchronization type.
    #[serde(rename = "write_unstable_reply")]
    pub write_unstable_reply: Option<String>,
    /// Specifies the zone in which the export is valid.
    #[serde(rename = "zone")]
    pub zone: Option<String>,
}
