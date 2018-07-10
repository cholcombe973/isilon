#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncPolicyExtended {
    /// If set to true, SyncIQ will perform failback configuration tasks during the next job run, rather than waiting to perform those tasks during the failback process. Performing these tasks ahead of time will increase the speed of failback operations.
    #[serde(rename = "accelerated_failback")]
    pub accelerated_failback: bool,
    /// If 'copy', source files will be copied to the target cluster.  If 'sync', the target directory will be made an image of the source directory:  Files and directories that have been deleted on the source, have been moved within the target directory, or no longer match the selection criteria will be deleted from the target directory.
    #[serde(rename = "action")]
    pub action: String,
    /// NOTE: This field should not be changed without the help of Isilon support.  Enable/disable UDP-based data transfer.
    #[serde(rename = "burst_mode")]
    pub burst_mode: bool,
    /// If true, retain previous source snapshot and incremental repstate, both of which are required for changelist creation.
    #[serde(rename = "changelist")]
    pub changelist: bool,
    /// If true, the sync target performs cyclic redundancy checks (CRC) on the data as it is received.
    #[serde(rename = "check_integrity")]
    pub check_integrity: bool,
    /// If set to deny, replicates all CloudPools smartlinks to the target cluster as smartlinks; if the target cluster does not support the smartlinks, the job will fail. If set to force, replicates all smartlinks to the target cluster as regular files. If set to allow, SyncIQ will attempt to replicate smartlinks to the target cluster as smartlinks; if the target cluster does not support the smartlinks, SyncIQ will replicate the smartlinks as regular files.
    #[serde(rename = "cloud_deep_copy")]
    pub cloud_deep_copy: String,
    /// NOTE: This field should not be changed without the help of Isilon support.  If true, the most recent run of this policy encountered an error and this policy will not start any more scheduled jobs until this field is manually set back to 'false'.
    #[serde(rename = "conflicted")]
    pub conflicted: bool,
    /// User-assigned description of this sync policy.
    #[serde(rename = "description")]
    pub description: String,
    /// NOTE: This field should not be changed without the help of Isilon support.  If true, the 7.2+ file splitting capability will be disabled.
    #[serde(rename = "disable_file_split")]
    pub disable_file_split: bool,
    /// NOTE: This field should not be changed without the help of Isilon support.  Enable/disable sync failover/failback.
    #[serde(rename = "disable_fofb")]
    pub disable_fofb: bool,
    /// NOTE: This field should not be changed without the help of Isilon support.  Enable/disable the 6.5+ STF based data transfer and uses only treewalk.
    #[serde(rename = "disable_stf")]
    pub disable_stf: bool,
    /// If true, jobs will be automatically run based on this policy, according to its schedule.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// NOTE: This field should not be changed without the help of Isilon support.  Continue sending files even with the corrupted filesystem.
    #[serde(rename = "expected_dataloss")]
    pub expected_dataloss: bool,
    /// A file matching pattern, organized as an OR'ed set of AND'ed file criteria, for example ((a AND b) OR (x AND y)) used to define a set of files with specific properties.  Policies of type 'sync' cannot use 'path' or time criteria in their matching patterns, but policies of type 'copy' can use all listed criteria.
    #[serde(rename = "file_matching_pattern")]
    pub file_matching_pattern: ::models::SyncJobPolicyFileMatchingPattern,
    /// NOTE: This field should not be changed without the help of Isilon support.  Determines whether data is sent only through the subnet and pool specified in the \"source_network\" field. This option can be useful if there are multiple interfaces for the given source subnet.  If you enable this option, the net.inet.ip.choose_ifa_by_ipsrc sysctl should be set.
    #[serde(rename = "force_interface")]
    pub force_interface: bool,
    /// This field is false if the policy is in its initial sync state and true otherwise.  Setting this field to false will reset the policy's sync state.
    #[serde(rename = "has_sync_state")]
    pub has_sync_state: bool,
    /// The system ID given to this sync policy.
    #[serde(rename = "id")]
    pub id: String,
    /// If --schedule is set to When-Source-Modified, the duration to wait after a modification is made before starting a job (default is 0 seconds).
    #[serde(rename = "job_delay")]
    pub job_delay: Option<i32>,
    /// This is the state of the most recent job for this policy.
    #[serde(rename = "last_job_state")]
    pub last_job_state: String,
    /// The most recent time a job was started for this policy.  Value is null if the policy has never been run.
    #[serde(rename = "last_started")]
    pub last_started: Option<i32>,
    /// Timestamp of last known successfully completed synchronization.  Value is null if the policy has never completed successfully.
    #[serde(rename = "last_success")]
    pub last_success: Option<i32>,
    /// Severity an event must reach before it is logged.
    #[serde(rename = "log_level")]
    pub log_level: String,
    /// If true, the system will log any files or directories that are deleted due to a sync.
    #[serde(rename = "log_removed_files")]
    pub log_removed_files: bool,
    /// User-assigned name of this sync policy.
    #[serde(rename = "name")]
    pub name: String,
    /// This is the next time a job is scheduled to run for this policy in Unix epoch seconds.  This field is null if the job is not scheduled.
    #[serde(rename = "next_run")]
    pub next_run: Option<i32>,
    /// Indicates if a password is set for accessing the target cluster. Password value is not shown with GET.
    #[serde(rename = "password_set")]
    pub password_set: bool,
    /// Determines the priority level of a policy. Policies with higher priority will have precedence to run over lower priority policies. Valid range is [0, 1]. Default is 0.
    #[serde(rename = "priority")]
    pub priority: i32,
    /// Length of time (in seconds) a policy report will be stored.
    #[serde(rename = "report_max_age")]
    pub report_max_age: i32,
    /// Maximum number of policy reports that will be stored on the system.
    #[serde(rename = "report_max_count")]
    pub report_max_count: i32,
    /// If you specify true, and you specify a SmartConnect zone in the \"target_host\" field, replication policies will connect only to nodes in the specified SmartConnect zone.  If you specify false, replication policies are not restricted to specific nodes on the target cluster.
    #[serde(rename = "restrict_target_network")]
    pub restrict_target_network: bool,
    /// If --schedule is set to a time/date, an alert is created if the specified RPO for this policy is exceeded. The default value is 0, which will not generate RPO alerts.
    #[serde(rename = "rpo_alert")]
    pub rpo_alert: Option<i32>,
    /// The schedule on which new jobs will be run for this policy.
    #[serde(rename = "schedule")]
    pub schedule: String,
    /// Skip DNS lookup of target IPs.
    #[serde(rename = "skip_lookup")]
    pub skip_lookup: bool,
    /// If true and --schedule is set to a time/date, the policy will not run if no changes have been made to the contents of the source directory since the last job successfully completed.
    #[serde(rename = "skip_when_source_unmodified")]
    pub skip_when_source_unmodified: bool,
    /// If true, snapshot-triggered syncs will include snapshots taken before policy creation time (requires --schedule when-snapshot-taken).
    #[serde(rename = "snapshot_sync_existing")]
    pub snapshot_sync_existing: bool,
    #[serde(rename = "snapshot_sync_pattern")]
    pub snapshot_sync_pattern: String,
    /// Directories that will be excluded from the sync.  Modifying this field will result in a full synchronization of all data.
    #[serde(rename = "source_exclude_directories")]
    pub source_exclude_directories: Vec<String>,
    /// Directories that will be included in the sync.  Modifying this field will result in a full synchronization of all data.
    #[serde(rename = "source_include_directories")]
    pub source_include_directories: Vec<String>,
    /// Restricts replication policies on the local cluster to running on the specified subnet and pool.
    #[serde(rename = "source_network")]
    pub source_network: Option<::models::SyncPolicySourceNetwork>,
    /// The root directory on the source cluster the files will be synced from.  Modifying this field will result in a full synchronization of all data.
    #[serde(rename = "source_root_path")]
    pub source_root_path: String,
    /// If true, archival snapshots of the source data will be taken on the source cluster before a sync.
    #[serde(rename = "source_snapshot_archive")]
    pub source_snapshot_archive: bool,
    /// The length of time in seconds to keep snapshots on the source cluster.
    #[serde(rename = "source_snapshot_expiration")]
    pub source_snapshot_expiration: i32,
    /// The name pattern for snapshots taken on the source cluster before a sync.
    #[serde(rename = "source_snapshot_pattern")]
    pub source_snapshot_pattern: String,
    /// If true, the target creates diffs against the original sync.
    #[serde(rename = "target_compare_initial_sync")]
    pub target_compare_initial_sync: bool,
    /// If true, target cluster will detect if files have been changed on the target by legacy tree walk syncs.
    #[serde(rename = "target_detect_modifications")]
    pub target_detect_modifications: bool,
    /// Hostname or IP address of sync target cluster.  Modifying the target cluster host can result in the policy being unrunnable if the new target does not match the current target association.
    #[serde(rename = "target_host")]
    pub target_host: String,
    /// Absolute filesystem path on the target cluster for the sync destination.
    #[serde(rename = "target_path")]
    pub target_path: String,
    /// The alias of the snapshot taken on the target cluster after the sync completes. A value of @DEFAULT will reset this field to the default creation value.
    #[serde(rename = "target_snapshot_alias")]
    pub target_snapshot_alias: String,
    /// If true, archival snapshots of the target data will be taken on the target cluster after successful sync completions.
    #[serde(rename = "target_snapshot_archive")]
    pub target_snapshot_archive: bool,
    /// The length of time in seconds to keep snapshots on the target cluster.
    #[serde(rename = "target_snapshot_expiration")]
    pub target_snapshot_expiration: i32,
    /// The name pattern for snapshots taken on the target cluster after the sync completes.  A value of @DEFAULT will reset this field to the default creation value.
    #[serde(rename = "target_snapshot_pattern")]
    pub target_snapshot_pattern: String,
    /// The number of worker threads on a node performing a sync.
    #[serde(rename = "workers_per_node")]
    pub workers_per_node: i32,
}
