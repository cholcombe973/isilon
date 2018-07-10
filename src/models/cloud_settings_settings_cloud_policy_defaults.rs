

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudSettingsSettingsCloudPolicyDefaults {
  /// Specifies if files with snapshots should be archived.
  #[serde(rename = "archive_snapshot_files")]
  archive_snapshot_files: Option<bool>,
  /// Specifies default cloudpool cache settings for new filepool policies.
  #[serde(rename = "cache")]
  cache: Option<::models::CloudSettingsSettingsCloudPolicyDefaultsCache>,
  /// Specifies if files should be compressed.
  #[serde(rename = "compression")]
  compression: Option<bool>,
  /// Specifies the minimum amount of time archived data will be retained in the cloud after deletion.
  #[serde(rename = "data_retention")]
  data_retention: Option<i32>,
  /// Specifies if files should be encrypted.
  #[serde(rename = "encryption")]
  encryption: Option<bool>,
  /// (Used with NDMP backups only.  Not applicable to SyncIQ.)  The minimum amount of time cloud files will be retained after the creation of a full NDMP backup.
  #[serde(rename = "full_backup_retention")]
  full_backup_retention: Option<i32>,
  /// (Used with SyncIQ and NDMP backups.)  The minimum amount of time cloud files will be retained after the creation of a SyncIQ backup or an incremental NDMP backup.
  #[serde(rename = "incremental_backup_retention")]
  incremental_backup_retention: Option<i32>,
  /// The minimum amount of time to wait before updating cloud data with local changes.
  #[serde(rename = "writeback_frequency")]
  writeback_frequency: Option<i32>
}

