

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NfsSettingsExportSettings {
  /// True if all directories under the specified paths are mountable.
  #[serde(rename = "all_dirs")]
  all_dirs: Option<bool>,
  /// Specifies the block size returned by the NFS statfs procedure.
  #[serde(rename = "block_size")]
  block_size: Option<i32>,
  /// True if the client can set file times through the NFS set attribute request. This parameter does not affect server behavior, but is included to accommoate legacy client requirements.
  #[serde(rename = "can_set_time")]
  can_set_time: Option<bool>,
  /// True if the case is ignored for file names. This parameter does not affect server behavior, but is included to accommodate legacy client requirements.
  #[serde(rename = "case_insensitive")]
  case_insensitive: Option<bool>,
  /// True if the case is preserved for file names. This parameter does not affect server behavior, but is included to accommodate legacy client requirements.
  #[serde(rename = "case_preserving")]
  case_preserving: Option<bool>,
  /// True if the superuser can change file ownership. This parameter does not affect server behavior, but is included to accommodate legacy client requirements.
  #[serde(rename = "chown_restricted")]
  chown_restricted: Option<bool>,
  /// True if NFS  commit  requests execute asynchronously.
  #[serde(rename = "commit_asynchronous")]
  commit_asynchronous: Option<bool>,
  /// Specifies the preferred size for directory read operations. This value is used to advise the client of optimal settings for the server, but is not enforced.
  #[serde(rename = "directory_transfer_size")]
  directory_transfer_size: Option<i32>,
  /// Specifies the default character set encoding of the clients connecting to the export, unless otherwise specified.
  #[serde(rename = "encoding")]
  encoding: Option<String>,
  /// Specifies the reported maximum number of links to a file. This parameter does not affect server behavior, but is included to accommodate legacy client requirements.
  #[serde(rename = "link_max")]
  link_max: Option<i32>,
  /// User and group mapping.
  #[serde(rename = "map_all")]
  map_all: Option<::models::NfsSettingsExportSettingsMapAll>,
  /// User and group mapping.
  #[serde(rename = "map_failure")]
  map_failure: Option<::models::NfsSettingsExportSettingsMapAll>,
  /// True if user mappings query the OneFS user database. When set to false, user mappings only query local authentication.
  #[serde(rename = "map_full")]
  map_full: Option<bool>,
  /// True if incoming user IDs (UIDs) are mapped to users in the OneFS user database. When set to false, incoming UIDs are applied directly to file operations.
  #[serde(rename = "map_lookup_uid")]
  map_lookup_uid: Option<bool>,
  /// User and group mapping.
  #[serde(rename = "map_non_root")]
  map_non_root: Option<::models::NfsSettingsExportSettingsMapAll>,
  /// Determines whether searches for users specified in 'map_all', 'map_root' or 'map_nonroot' are retried if the search fails.
  #[serde(rename = "map_retry")]
  map_retry: Option<bool>,
  /// User and group mapping.
  #[serde(rename = "map_root")]
  map_root: Option<::models::NfsSettingsExportSettingsMapAll>,
  /// Specifies the maximum file size for any file accessed from the export. This parameter does not affect server behavior, but is included to accommodate legacy client requirements.
  #[serde(rename = "max_file_size")]
  max_file_size: Option<i32>,
  /// Specifies the reported maximum length of a file name. This parameter does not affect server behavior, but is included to accommodate legacy client requirements.
  #[serde(rename = "name_max_size")]
  name_max_size: Option<i32>,
  /// True if long file names result in an error. This parameter does not affect server behavior, but is included to accommodate legacy client requirements.
  #[serde(rename = "no_truncate")]
  no_truncate: Option<bool>,
  /// True if the export is set to read-only.
  #[serde(rename = "read_only")]
  read_only: Option<bool>,
  /// Specifies the maximum buffer size that clients should use on NFS read requests. This value is used to advise the client of optimal settings for the server, but is not enforced.
  #[serde(rename = "read_transfer_max_size")]
  read_transfer_max_size: Option<i32>,
  /// Specifies the preferred multiple size for NFS read requests. This value is used to advise the client of optimal settings for the server, but is not enforced.
  #[serde(rename = "read_transfer_multiple")]
  read_transfer_multiple: Option<i32>,
  /// Specifies the preferred size for NFS read requests. This value is used to advise the client of optimal settings for the server, but is not enforced.
  #[serde(rename = "read_transfer_size")]
  read_transfer_size: Option<i32>,
  /// True if 'readdirplus' requests are enabled. Enabling this property might improve network performance and is only available for NFSv3.
  #[serde(rename = "readdirplus")]
  readdirplus: Option<bool>,
  /// Sets the number of directory entries that are prefetched when a 'readdirplus' request is processed. (Deprecated.)
  #[serde(rename = "readdirplus_prefetch")]
  readdirplus_prefetch: Option<i32>,
  /// Limits the size of file identifiers returned by NFSv3+ to 32-bit values (may require remount).
  #[serde(rename = "return_32bit_file_ids")]
  return_32bit_file_ids: Option<bool>,
  /// Specifies the authentication types that are supported for this export.
  #[serde(rename = "security_flavors")]
  security_flavors: Option<Vec<String>>,
  /// True if set attribute operations execute asynchronously.
  #[serde(rename = "setattr_asynchronous")]
  setattr_asynchronous: Option<bool>,
  /// Specifies the snapshot for all mounts.
  #[serde(rename = "snapshot")]
  snapshot: Option<String>,
  /// True if symlinks are supported. This value is used to advise the client of optimal settings for the server, but is not enforced.
  #[serde(rename = "symlinks")]
  symlinks: Option<bool>,
  /// Specifies the resolution of all time values that are returned to the clients
  #[serde(rename = "time_delta")]
  time_delta: Option<f32>,
  /// Specifies the synchronization type.
  #[serde(rename = "write_datasync_action")]
  write_datasync_action: Option<String>,
  /// Specifies the synchronization type.
  #[serde(rename = "write_datasync_reply")]
  write_datasync_reply: Option<String>,
  /// Specifies the synchronization type.
  #[serde(rename = "write_filesync_action")]
  write_filesync_action: Option<String>,
  /// Specifies the synchronization type.
  #[serde(rename = "write_filesync_reply")]
  write_filesync_reply: Option<String>,
  /// Specifies the maximum buffer size that clients should use on NFS write requests. This value is used to advise the client of optimal settings for the server, but is not enforced.
  #[serde(rename = "write_transfer_max_size")]
  write_transfer_max_size: Option<i32>,
  /// Specifies the preferred multiple size for NFS write requests. This value is used to advise the client of optimal settings for the server, but is not enforced.
  #[serde(rename = "write_transfer_multiple")]
  write_transfer_multiple: Option<i32>,
  /// Specifies the preferred multiple size for NFS write requests. This value is used to advise the client of optimal settings for the server, but is not enforced.
  #[serde(rename = "write_transfer_size")]
  write_transfer_size: Option<i32>,
  /// Specifies the synchronization type.
  #[serde(rename = "write_unstable_action")]
  write_unstable_action: Option<String>,
  /// Specifies the synchronization type.
  #[serde(rename = "write_unstable_reply")]
  write_unstable_reply: Option<String>,
  /// Specifies the zone in which the export is valid.
  #[serde(rename = "zone")]
  zone: Option<String>
}

