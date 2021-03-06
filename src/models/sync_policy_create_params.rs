#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncPolicyCreateParams {
    /// If set to true, SyncIQ will perform failback configuration tasks during the next job run, rather than waiting to perform those tasks during the failback process. Performing these tasks ahead of time will increase the speed of failback operations.
    #[serde(rename = "accelerated_failback")]
    pub accelerated_failback: Option<bool>,
    /// If 'copy', source files will be copied to the target cluster.  If 'sync', the target directory will be made an image of the source directory:  Files and directories that have been deleted on the source, have been moved within the target directory, or no longer match the selection criteria will be deleted from the target directory.
    #[serde(rename = "action")]
    pub action: String,
    /// NOTE: This field should not be changed without the help of Isilon support.  Enable/disable UDP-based data transfer.
    #[serde(rename = "burst_mode")]
    pub burst_mode: Option<bool>,
    /// If true, retain previous source snapshot and incremental repstate, both of which are required for changelist creation.
    #[serde(rename = "changelist")]
    pub changelist: Option<bool>,
    /// If true, the sync target performs cyclic redundancy checks (CRC) on the data as it is received.
    #[serde(rename = "check_integrity")]
    pub check_integrity: Option<bool>,
    /// If set to deny, replicates all CloudPools smartlinks to the target cluster as smartlinks; if the target cluster does not support the smartlinks, the job will fail. If set to force, replicates all smartlinks to the target cluster as regular files. If set to allow, SyncIQ will attempt to replicate smartlinks to the target cluster as smartlinks; if the target cluster does not support the smartlinks, SyncIQ will replicate the smartlinks as regular files.
    #[serde(rename = "cloud_deep_copy")]
    pub cloud_deep_copy: Option<String>,
    /// User-assigned description of this sync policy.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// NOTE: This field should not be changed without the help of Isilon support.  If true, the 7.2+ file splitting capability will be disabled.
    #[serde(rename = "disable_file_split")]
    pub disable_file_split: Option<bool>,
    /// NOTE: This field should not be changed without the help of Isilon support.  Enable/disable sync failover/failback.
    #[serde(rename = "disable_fofb")]
    pub disable_fofb: Option<bool>,
    /// NOTE: This field should not be changed without the help of Isilon support.  Enable/disable the 6.5+ STF based data transfer and uses only treewalk.
    #[serde(rename = "disable_stf")]
    pub disable_stf: Option<bool>,
    /// If true, jobs will be automatically run based on this policy, according to its schedule.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// NOTE: This field should not be changed without the help of Isilon support.  Continue sending files even with the corrupted filesystem.
    #[serde(rename = "expected_dataloss")]
    pub expected_dataloss: Option<bool>,
    /// A file matching pattern, organized as an OR'ed set of AND'ed file criteria, for example ((a AND b) OR (x AND y)) used to define a set of files with specific properties.  Policies of type 'sync' cannot use 'path' or time criteria in their matching patterns, but policies of type 'copy' can use all listed criteria.
    #[serde(rename = "file_matching_pattern")]
    pub file_matching_pattern: Option<crate::models::SyncJobPolicyFileMatchingPattern>,
    /// NOTE: This field should not be changed without the help of Isilon support.  Determines whether data is sent only through the subnet and pool specified in the \"source_network\" field. This option can be useful if there are multiple interfaces for the given source subnet.  If you enable this option, the net.inet.ip.choose_ifa_by_ipsrc sysctl should be set.
    #[serde(rename = "force_interface")]
    pub force_interface: Option<bool>,
    /// If --schedule is set to When-Source-Modified, the duration to wait after a modification is made before starting a job (default is 0 seconds).
    #[serde(rename = "job_delay")]
    pub job_delay: Option<i32>,
    /// Severity an event must reach before it is logged.
    #[serde(rename = "log_level")]
    pub log_level: Option<String>,
    /// If true, the system will log any files or directories that are deleted due to a sync.
    #[serde(rename = "log_removed_files")]
    pub log_removed_files: Option<bool>,
    /// User-assigned name of this sync policy.
    #[serde(rename = "name")]
    pub name: String,
    /// The password for the target cluster.  This field is not readable.
    #[serde(rename = "password")]
    pub password: Option<String>,
    /// Determines the priority level of a policy. Policies with higher priority will have precedence to run over lower priority policies. Valid range is [0, 1]. Default is 0.
    #[serde(rename = "priority")]
    pub priority: Option<i32>,
    /// Length of time (in seconds) a policy report will be stored.
    #[serde(rename = "report_max_age")]
    pub report_max_age: Option<i32>,
    /// Maximum number of policy reports that will be stored on the system.
    #[serde(rename = "report_max_count")]
    pub report_max_count: Option<i32>,
    /// If you specify true, and you specify a SmartConnect zone in the \"target_host\" field, replication policies will connect only to nodes in the specified SmartConnect zone.  If you specify false, replication policies are not restricted to specific nodes on the target cluster.
    #[serde(rename = "restrict_target_network")]
    pub restrict_target_network: Option<bool>,
    /// If --schedule is set to a time/date, an alert is created if the specified RPO for this policy is exceeded. The default value is 0, which will not generate RPO alerts.
    #[serde(rename = "rpo_alert")]
    pub rpo_alert: Option<i32>,
    /// The schedule on which new jobs will be run for this policy.
    #[serde(rename = "schedule")]
    pub schedule: Option<String>,
    /// Skip DNS lookup of target IPs.
    #[serde(rename = "skip_lookup")]
    pub skip_lookup: Option<bool>,
    /// If true and --schedule is set to a time/date, the policy will not run if no changes have been made to the contents of the source directory since the last job successfully completed.
    #[serde(rename = "skip_when_source_unmodified")]
    pub skip_when_source_unmodified: Option<bool>,
    /// If true, snapshot-triggered syncs will include snapshots taken before policy creation time (requires --schedule when-snapshot-taken).
    #[serde(rename = "snapshot_sync_existing")]
    pub snapshot_sync_existing: Option<bool>,
    #[serde(rename = "snapshot_sync_pattern")]
    pub snapshot_sync_pattern: Option<String>,
    /// Directories that will be excluded from the sync.  Modifying this field will result in a full synchronization of all data.
    #[serde(rename = "source_exclude_directories")]
    pub source_exclude_directories: Option<Vec<String>>,
    /// Directories that will be included in the sync.  Modifying this field will result in a full synchronization of all data.
    #[serde(rename = "source_include_directories")]
    pub source_include_directories: Option<Vec<String>>,
    /// Restricts replication policies on the local cluster to running on the specified subnet and pool.
    #[serde(rename = "source_network")]
    pub source_network: Option<crate::models::SyncPolicySourceNetwork>,
    /// The root directory on the source cluster the files will be synced from.  Modifying this field will result in a full synchronization of all data.
    #[serde(rename = "source_root_path")]
    pub source_root_path: String,
    /// If true, archival snapshots of the source data will be taken on the source cluster before a sync.
    #[serde(rename = "source_snapshot_archive")]
    pub source_snapshot_archive: Option<bool>,
    /// The length of time in seconds to keep snapshots on the source cluster.
    #[serde(rename = "source_snapshot_expiration")]
    pub source_snapshot_expiration: Option<i32>,
    /// The name pattern for snapshots taken on the source cluster before a sync.
    #[serde(rename = "source_snapshot_pattern")]
    pub source_snapshot_pattern: Option<String>,
    /// If true, the target creates diffs against the original sync.
    #[serde(rename = "target_compare_initial_sync")]
    pub target_compare_initial_sync: Option<bool>,
    /// If true, target cluster will detect if files have been changed on the target by legacy tree walk syncs.
    #[serde(rename = "target_detect_modifications")]
    pub target_detect_modifications: Option<bool>,
    /// Hostname or IP address of sync target cluster.  Modifying the target cluster host can result in the policy being unrunnable if the new target does not match the current target association.
    #[serde(rename = "target_host")]
    pub target_host: String,
    /// Absolute filesystem path on the target cluster for the sync destination.
    #[serde(rename = "target_path")]
    pub target_path: String,
    /// The alias of the snapshot taken on the target cluster after the sync completes. A value of @DEFAULT will reset this field to the default creation value.
    #[serde(rename = "target_snapshot_alias")]
    pub target_snapshot_alias: Option<String>,
    /// If true, archival snapshots of the target data will be taken on the target cluster after successful sync completions.
    #[serde(rename = "target_snapshot_archive")]
    pub target_snapshot_archive: Option<bool>,
    /// The length of time in seconds to keep snapshots on the target cluster.
    #[serde(rename = "target_snapshot_expiration")]
    pub target_snapshot_expiration: Option<i32>,
    /// The name pattern for snapshots taken on the target cluster after the sync completes.  A value of @DEFAULT will reset this field to the default creation value.
    #[serde(rename = "target_snapshot_pattern")]
    pub target_snapshot_pattern: Option<String>,
    /// The number of worker threads on a node performing a sync.
    #[serde(rename = "workers_per_node")]
    pub workers_per_node: Option<i32>,
}
