#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportsReportSubreportsSubreport {
    /// The action to be taken by this job.
    #[serde(rename = "action")]
    pub action: String,
    /// The number of ads streams replicated by this job.
    #[serde(rename = "ads_streams_replicated")]
    pub ads_streams_replicated: u64,
    /// The number of block specs replicated by this job.
    #[serde(rename = "block_specs_replicated")]
    pub block_specs_replicated: u64,
    /// The number of bytes recoverable by this job.
    #[serde(rename = "bytes_recoverable")]
    pub bytes_recoverable: u64,
    /// The number of bytes that have been transferred by this job.
    #[serde(rename = "bytes_transferred")]
    pub bytes_transferred: u64,
    /// The number of char specs replicated by this job.
    #[serde(rename = "char_specs_replicated")]
    pub char_specs_replicated: u64,
    /// The number of LINs corrected by this job.
    #[serde(rename = "corrected_lins")]
    pub corrected_lins: u64,
    /// This field is true if the node running this job is dead.
    #[serde(rename = "dead_node")]
    pub dead_node: bool,
    /// The number of directories replicated.
    #[serde(rename = "directories_replicated")]
    pub directories_replicated: u64,
    /// The number of directories changed by this job.
    #[serde(rename = "dirs_changed")]
    pub dirs_changed: u64,
    /// The number of directories deleted by this job.
    #[serde(rename = "dirs_deleted")]
    pub dirs_deleted: u64,
    /// The number of directories moved by this job.
    #[serde(rename = "dirs_moved")]
    pub dirs_moved: u64,
    /// The number of directories created by this job.
    #[serde(rename = "dirs_new")]
    pub dirs_new: u64,
    /// The amount of time in seconds between when the job was started and when it ended.  If the job has not yet ended, this is the amount of time since the job started.  This field is null if the job has not yet started.
    #[serde(rename = "duration")]
    pub duration: Option<u64>,
    /// The time the job ended in unix epoch seconds. The field is null if the job hasn't ended.
    #[serde(rename = "end_time")]
    pub end_time: Option<u64>,
    /// The primary error message for this job.
    #[serde(rename = "error")]
    pub error: String,
    /// The number of files with checksum errors skipped by this job.
    #[serde(rename = "error_checksum_files_skipped")]
    pub error_checksum_files_skipped: u64,
    /// The number of files with io errors skipped by this job.
    #[serde(rename = "error_io_files_skipped")]
    pub error_io_files_skipped: u64,
    /// The number of files with network errors skipped by this job.
    #[serde(rename = "error_net_files_skipped")]
    pub error_net_files_skipped: u64,
    /// A list of error messages for this job.
    #[serde(rename = "errors")]
    pub errors: Vec<String>,
    /// Tyhe number of data chunks that failed transmission.
    #[serde(rename = "failed_chunks")]
    pub failed_chunks: u64,
    /// The number of fifos replicated by this job.
    #[serde(rename = "fifos_replicated")]
    pub fifos_replicated: u64,
    /// The number of bytes transferred that belong to files.
    #[serde(rename = "file_data_bytes")]
    pub file_data_bytes: u64,
    /// The number of files changed by this job.
    #[serde(rename = "files_changed")]
    pub files_changed: u64,
    /// The number of files linked by this job.
    #[serde(rename = "files_linked")]
    pub files_linked: u64,
    /// The number of files created by this job.
    #[serde(rename = "files_new")]
    pub files_new: u64,
    /// The number of files selected by this job.
    #[serde(rename = "files_selected")]
    pub files_selected: u64,
    /// The number of files transferred by this job.
    #[serde(rename = "files_transferred")]
    pub files_transferred: u64,
    /// The number of files unlinked by this job.
    #[serde(rename = "files_unlinked")]
    pub files_unlinked: u64,
    /// The number of files with ads replicated by this job.
    #[serde(rename = "files_with_ads_replicated")]
    pub files_with_ads_replicated: u64,
    /// The number of LINs flipped by this job.
    #[serde(rename = "flipped_lins")]
    pub flipped_lins: u64,
    /// The number of hard links replicated by this job.
    #[serde(rename = "hard_links_replicated")]
    pub hard_links_replicated: u64,
    /// The number of hash exceptions fixed by this job.
    #[serde(rename = "hash_exceptions_fixed")]
    pub hash_exceptions_fixed: u64,
    /// The number of hash exceptions found by this job.
    #[serde(rename = "hash_exceptions_found")]
    pub hash_exceptions_found: u64,
    /// A unique identifier for this object.
    #[serde(rename = "id")]
    pub id: String,
    /// The ID of the job.
    #[serde(rename = "job_id")]
    pub job_id: Option<u64>,
    /// The number of LINs transferred by this job.
    #[serde(rename = "lins_total")]
    pub lins_total: u64,
    /// The total number of bytes sent to the source by this job.
    #[serde(rename = "network_bytes_to_source")]
    pub network_bytes_to_source: u64,
    /// The total number of bytes sent to the target by this job.
    #[serde(rename = "network_bytes_to_target")]
    pub network_bytes_to_target: u64,
    /// The number of new files replicated by this job.
    #[serde(rename = "new_files_replicated")]
    pub new_files_replicated: u64,
    /// The number of files that have been retransmitted by this job.
    #[serde(rename = "num_retransmitted_files")]
    pub num_retransmitted_files: u64,
    /// Data for each phase of this job.
    #[serde(rename = "phases")]
    pub phases: Vec<::models::SyncJobPhase>,
    /// The ID of the policy.
    #[serde(rename = "policy_id")]
    pub policy_id: String,
    /// The name of the policy.
    #[serde(rename = "policy_name")]
    pub policy_name: String,
    /// The number of regular files replicated by this job.
    #[serde(rename = "regular_files_replicated")]
    pub regular_files_replicated: u64,
    /// The number of LINs resynched by this job.
    #[serde(rename = "resynced_lins")]
    pub resynced_lins: u64,
    /// The files that have been retransmitted by this job.
    #[serde(rename = "retransmitted_files")]
    pub retransmitted_files: Vec<String>,
    /// The number of times the job has been retried.
    #[serde(rename = "retry")]
    pub retry: u64,
    /// The number of data chunks currently being transmitted.
    #[serde(rename = "running_chunks")]
    pub running_chunks: u64,
    /// The number of sockets replicated by this job.
    #[serde(rename = "sockets_replicated")]
    pub sockets_replicated: u64,
    /// The number of bytes recovered on the source.
    #[serde(rename = "source_bytes_recovered")]
    pub source_bytes_recovered: u64,
    /// The number of directories created on the source.
    #[serde(rename = "source_directories_created")]
    pub source_directories_created: u64,
    /// The number of directories deleted on the source.
    #[serde(rename = "source_directories_deleted")]
    pub source_directories_deleted: u64,
    /// The number of directories linked on the source.
    #[serde(rename = "source_directories_linked")]
    pub source_directories_linked: u64,
    /// The number of directories unlinked on the source.
    #[serde(rename = "source_directories_unlinked")]
    pub source_directories_unlinked: u64,
    /// The number of directories visited on the source.
    #[serde(rename = "source_directories_visited")]
    pub source_directories_visited: u64,
    /// The number of files deleted on the source.
    #[serde(rename = "source_files_deleted")]
    pub source_files_deleted: u64,
    /// The number of files linked on the source.
    #[serde(rename = "source_files_linked")]
    pub source_files_linked: u64,
    /// The number of files unlinked on the source.
    #[serde(rename = "source_files_unlinked")]
    pub source_files_unlinked: u64,
    /// Hostname or IP address of sync source cluster.
    #[serde(rename = "source_host")]
    pub source_host: String,
    /// The number of sparse data bytes transferred by this job.
    #[serde(rename = "sparse_data_bytes")]
    pub sparse_data_bytes: u64,
    /// The time the job started in unix epoch seconds. The field is null if the job hasn't started.
    #[serde(rename = "start_time")]
    pub start_time: Option<u64>,
    /// The state of the job.
    #[serde(rename = "state")]
    pub state: String,
    /// The number of data chunks that have been transmitted successfully.
    #[serde(rename = "succeeded_chunks")]
    pub succeeded_chunks: u64,
    /// The number of symlinks replicated by this job.
    #[serde(rename = "symlinks_replicated")]
    pub symlinks_replicated: u64,
    /// The type of sync being performed by this job.
    #[serde(rename = "sync_type")]
    pub sync_type: String,
    /// The number of bytes recovered on the target.
    #[serde(rename = "target_bytes_recovered")]
    pub target_bytes_recovered: u64,
    /// The number of directories created on the target.
    #[serde(rename = "target_directories_created")]
    pub target_directories_created: u64,
    /// The number of directories deleted on the target.
    #[serde(rename = "target_directories_deleted")]
    pub target_directories_deleted: u64,
    /// The number of directories linked on the target.
    #[serde(rename = "target_directories_linked")]
    pub target_directories_linked: u64,
    /// The number of directories unlinked on the target.
    #[serde(rename = "target_directories_unlinked")]
    pub target_directories_unlinked: u64,
    /// The number of files deleted on the target.
    #[serde(rename = "target_files_deleted")]
    pub target_files_deleted: u64,
    /// The number of files linked on the target.
    #[serde(rename = "target_files_linked")]
    pub target_files_linked: u64,
    /// The number of files unlinked on the target.
    #[serde(rename = "target_files_unlinked")]
    pub target_files_unlinked: u64,
    /// Absolute filesystem path on the target cluster for the sync destination.
    #[serde(rename = "target_path")]
    pub target_path: String,
    /// The target snapshots created by this job.
    #[serde(rename = "target_snapshots")]
    pub target_snapshots: Vec<String>,
    /// The total number of data chunks transmitted by this job.
    #[serde(rename = "total_chunks")]
    pub total_chunks: u64,
    /// The total number of bytes transferred by this job.
    #[serde(rename = "total_data_bytes")]
    pub total_data_bytes: u64,
    /// The number of files affected by this job.
    #[serde(rename = "total_files")]
    pub total_files: u64,
    /// The total number of bytes sent over the network by this job.
    #[serde(rename = "total_network_bytes")]
    pub total_network_bytes: u64,
    /// The total number of phases for this job.
    #[serde(rename = "total_phases")]
    pub total_phases: u64,
    /// The number of bytes unchanged by this job.
    #[serde(rename = "unchanged_data_bytes")]
    pub unchanged_data_bytes: u64,
    /// The number of up-to-date files skipped by this job.
    #[serde(rename = "up_to_date_files_skipped")]
    pub up_to_date_files_skipped: u64,
    /// The number of updated files replicated by this job.
    #[serde(rename = "updated_files_replicated")]
    pub updated_files_replicated: u64,
    /// The number of files with user conflicts skipped by this job.
    #[serde(rename = "user_conflict_files_skipped")]
    pub user_conflict_files_skipped: u64,
    /// A list of warning messages for this job.
    #[serde(rename = "warnings")]
    pub warnings: Vec<String>,
    /// The number of WORM committed files which needed to be reverted. Since WORM committed files cannot be reverted, this is the number of files that were preserved in the compliance store.
    #[serde(rename = "worm_committed_file_conflicts")]
    pub worm_committed_file_conflicts: u64,
}
