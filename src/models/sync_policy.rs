

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncPolicy {
  /// If set to true, SyncIQ will perform failback configuration tasks during the next job run, rather than waiting to perform those tasks during the failback process. Performing these tasks ahead of time will increase the speed of failback operations.
  #[serde(rename = "accelerated_failback")]
  accelerated_failback: Option<bool>,
  /// If 'copy', source files will be copied to the target cluster.  If 'sync', the target directory will be made an image of the source directory:  Files and directories that have been deleted on the source, have been moved within the target directory, or no longer match the selection criteria will be deleted from the target directory.
  #[serde(rename = "action")]
  action: Option<String>,
  /// NOTE: This field should not be changed without the help of Isilon support.  Enable/disable UDP-based data transfer.
  #[serde(rename = "burst_mode")]
  burst_mode: Option<bool>,
  /// If true, retain previous source snapshot and incremental repstate, both of which are required for changelist creation.
  #[serde(rename = "changelist")]
  changelist: Option<bool>,
  /// If true, the sync target performs cyclic redundancy checks (CRC) on the data as it is received.
  #[serde(rename = "check_integrity")]
  check_integrity: Option<bool>,
  /// If set to deny, replicates all CloudPools smartlinks to the target cluster as smartlinks; if the target cluster does not support the smartlinks, the job will fail. If set to force, replicates all smartlinks to the target cluster as regular files. If set to allow, SyncIQ will attempt to replicate smartlinks to the target cluster as smartlinks; if the target cluster does not support the smartlinks, SyncIQ will replicate the smartlinks as regular files.
  #[serde(rename = "cloud_deep_copy")]
  cloud_deep_copy: Option<String>,
  /// NOTE: This field should not be changed without the help of Isilon support.  If true, the most recent run of this policy encountered an error and this policy will not start any more scheduled jobs until this field is manually set back to 'false'.
  #[serde(rename = "conflicted")]
  conflicted: Option<bool>,
  /// User-assigned description of this sync policy.
  #[serde(rename = "description")]
  description: Option<String>,
  /// NOTE: This field should not be changed without the help of Isilon support.  If true, the 7.2+ file splitting capability will be disabled.
  #[serde(rename = "disable_file_split")]
  disable_file_split: Option<bool>,
  /// NOTE: This field should not be changed without the help of Isilon support.  Enable/disable sync failover/failback.
  #[serde(rename = "disable_fofb")]
  disable_fofb: Option<bool>,
  /// NOTE: This field should not be changed without the help of Isilon support.  Enable/disable the 6.5+ STF based data transfer and uses only treewalk.
  #[serde(rename = "disable_stf")]
  disable_stf: Option<bool>,
  /// If true, jobs will be automatically run based on this policy, according to its schedule.
  #[serde(rename = "enabled")]
  enabled: Option<bool>,
  /// NOTE: This field should not be changed without the help of Isilon support.  Continue sending files even with the corrupted filesystem.
  #[serde(rename = "expected_dataloss")]
  expected_dataloss: Option<bool>,
  /// A file matching pattern, organized as an OR'ed set of AND'ed file criteria, for example ((a AND b) OR (x AND y)) used to define a set of files with specific properties.  Policies of type 'sync' cannot use 'path' or time criteria in their matching patterns, but policies of type 'copy' can use all listed criteria.
  #[serde(rename = "file_matching_pattern")]
  file_matching_pattern: Option<::models::SyncJobPolicyFileMatchingPattern>,
  /// NOTE: This field should not be changed without the help of Isilon support.  Determines whether data is sent only through the subnet and pool specified in the \"source_network\" field. This option can be useful if there are multiple interfaces for the given source subnet.  If you enable this option, the net.inet.ip.choose_ifa_by_ipsrc sysctl should be set.
  #[serde(rename = "force_interface")]
  force_interface: Option<bool>,
  /// If --schedule is set to When-Source-Modified, the duration to wait after a modification is made before starting a job (default is 0 seconds).
  #[serde(rename = "job_delay")]
  job_delay: Option<i32>,
  /// Severity an event must reach before it is logged.
  #[serde(rename = "log_level")]
  log_level: Option<String>,
  /// If true, the system will log any files or directories that are deleted due to a sync.
  #[serde(rename = "log_removed_files")]
  log_removed_files: Option<bool>,
  /// User-assigned name of this sync policy.
  #[serde(rename = "name")]
  name: Option<String>,
  /// The password for the target cluster.  This field is not readable.
  #[serde(rename = "password")]
  password: Option<String>,
  /// Determines the priority level of a policy. Policies with higher priority will have precedence to run over lower priority policies. Valid range is [0, 1]. Default is 0.
  #[serde(rename = "priority")]
  priority: Option<i32>,
  /// Length of time (in seconds) a policy report will be stored.
  #[serde(rename = "report_max_age")]
  report_max_age: Option<i32>,
  /// Maximum number of policy reports that will be stored on the system.
  #[serde(rename = "report_max_count")]
  report_max_count: Option<i32>,
  /// If you specify true, and you specify a SmartConnect zone in the \"target_host\" field, replication policies will connect only to nodes in the specified SmartConnect zone.  If you specify false, replication policies are not restricted to specific nodes on the target cluster.
  #[serde(rename = "restrict_target_network")]
  restrict_target_network: Option<bool>,
  /// If --schedule is set to a time/date, an alert is created if the specified RPO for this policy is exceeded. The default value is 0, which will not generate RPO alerts.
  #[serde(rename = "rpo_alert")]
  rpo_alert: Option<i32>,
  /// The schedule on which new jobs will be run for this policy.
  #[serde(rename = "schedule")]
  schedule: Option<String>,
  /// Skip DNS lookup of target IPs.
  #[serde(rename = "skip_lookup")]
  skip_lookup: Option<bool>,
  /// If true and --schedule is set to a time/date, the policy will not run if no changes have been made to the contents of the source directory since the last job successfully completed.
  #[serde(rename = "skip_when_source_unmodified")]
  skip_when_source_unmodified: Option<bool>,
  /// If true, snapshot-triggered syncs will include snapshots taken before policy creation time (requires --schedule when-snapshot-taken).
  #[serde(rename = "snapshot_sync_existing")]
  snapshot_sync_existing: Option<bool>,
  #[serde(rename = "snapshot_sync_pattern")]
  snapshot_sync_pattern: Option<String>,
  /// Directories that will be excluded from the sync.  Modifying this field will result in a full synchronization of all data.
  #[serde(rename = "source_exclude_directories")]
  source_exclude_directories: Option<Vec<String>>,
  /// Directories that will be included in the sync.  Modifying this field will result in a full synchronization of all data.
  #[serde(rename = "source_include_directories")]
  source_include_directories: Option<Vec<String>>,
  /// Restricts replication policies on the local cluster to running on the specified subnet and pool.
  #[serde(rename = "source_network")]
  source_network: Option<::models::SyncPolicySourceNetwork>,
  /// The root directory on the source cluster the files will be synced from.  Modifying this field will result in a full synchronization of all data.
  #[serde(rename = "source_root_path")]
  source_root_path: Option<String>,
  /// If true, archival snapshots of the source data will be taken on the source cluster before a sync.
  #[serde(rename = "source_snapshot_archive")]
  source_snapshot_archive: Option<bool>,
  /// The length of time in seconds to keep snapshots on the source cluster.
  #[serde(rename = "source_snapshot_expiration")]
  source_snapshot_expiration: Option<i32>,
  /// The name pattern for snapshots taken on the source cluster before a sync.
  #[serde(rename = "source_snapshot_pattern")]
  source_snapshot_pattern: Option<String>,
  /// If true, the target creates diffs against the original sync.
  #[serde(rename = "target_compare_initial_sync")]
  target_compare_initial_sync: Option<bool>,
  /// If true, target cluster will detect if files have been changed on the target by legacy tree walk syncs.
  #[serde(rename = "target_detect_modifications")]
  target_detect_modifications: Option<bool>,
  /// Hostname or IP address of sync target cluster.  Modifying the target cluster host can result in the policy being unrunnable if the new target does not match the current target association.
  #[serde(rename = "target_host")]
  target_host: Option<String>,
  /// Absolute filesystem path on the target cluster for the sync destination.
  #[serde(rename = "target_path")]
  target_path: Option<String>,
  /// The alias of the snapshot taken on the target cluster after the sync completes. A value of @DEFAULT will reset this field to the default creation value.
  #[serde(rename = "target_snapshot_alias")]
  target_snapshot_alias: Option<String>,
  /// If true, archival snapshots of the target data will be taken on the target cluster after successful sync completions.
  #[serde(rename = "target_snapshot_archive")]
  target_snapshot_archive: Option<bool>,
  /// The length of time in seconds to keep snapshots on the target cluster.
  #[serde(rename = "target_snapshot_expiration")]
  target_snapshot_expiration: Option<i32>,
  /// The name pattern for snapshots taken on the target cluster after the sync completes.  A value of @DEFAULT will reset this field to the default creation value.
  #[serde(rename = "target_snapshot_pattern")]
  target_snapshot_pattern: Option<String>,
  /// The number of worker threads on a node performing a sync.
  #[serde(rename = "workers_per_node")]
  workers_per_node: Option<i32>
}

