

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncReportPolicy {
  /// If 'copy', source files will be copied to the target cluster.  If 'sync', the target directory will be made an image of the source directory:  Files and directories that have been deleted on the source, have been moved within the target directory, or no longer match the selection criteria will be deleted from the target directory.
  #[serde(rename = "action")]
  action: Option<String>,
  /// A file matching pattern, organized as an OR'ed set of AND'ed file criteria, for example ((a AND b) OR (x AND y)) used to define a set of files with specific properties.  Policies of type 'sync' cannot use 'path' or time criteria in their matching patterns, but policies of type 'copy' can use all listed criteria.
  #[serde(rename = "file_matching_pattern")]
  file_matching_pattern: Option<::models::SyncJobPolicyFileMatchingPattern>,
  /// User-assigned name of this sync policy.
  #[serde(rename = "name")]
  name: Option<String>,
  /// Directories that will be excluded from the sync.  Modifying this field will result in a full synchronization of all data.
  #[serde(rename = "source_exclude_directories")]
  source_exclude_directories: Option<Vec<String>>,
  /// Directories that will be included in the sync.  Modifying this field will result in a full synchronization of all data.
  #[serde(rename = "source_include_directories")]
  source_include_directories: Option<Vec<String>>,
  /// The root directory on the source cluster the files will be synced from.  Modifying this field will result in a full synchronization of all data.
  #[serde(rename = "source_root_path")]
  source_root_path: Option<String>,
  /// Hostname or IP address of sync target cluster.  Modifying the target cluster host can result in the policy being unrunnable if the new target does not match the current target association.
  #[serde(rename = "target_host")]
  target_host: Option<String>,
  /// Absolute filesystem path on the target cluster for the sync destination.
  #[serde(rename = "target_path")]
  target_path: Option<String>
}

