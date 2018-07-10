

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncReport {
  /// The action to be taken by this job.
  #[serde(rename = "action")]
  action: String,
  /// The number of ads streams replicated by this job.
  #[serde(rename = "ads_streams_replicated")]
  ads_streams_replicated: i32,
  /// The number of block specs replicated by this job.
  #[serde(rename = "block_specs_replicated")]
  block_specs_replicated: i32,
  /// The number of bytes recoverable by this job.
  #[serde(rename = "bytes_recoverable")]
  bytes_recoverable: i32,
  /// The number of bytes that have been transferred by this job.
  #[serde(rename = "bytes_transferred")]
  bytes_transferred: i32,
  /// The number of char specs replicated by this job.
  #[serde(rename = "char_specs_replicated")]
  char_specs_replicated: i32,
  /// The number of LINs corrected by this job.
  #[serde(rename = "corrected_lins")]
  corrected_lins: i32,
  /// This field is true if the node running this job is dead.
  #[serde(rename = "dead_node")]
  dead_node: bool,
  /// The number of directories replicated.
  #[serde(rename = "directories_replicated")]
  directories_replicated: i32,
  /// The number of directories changed by this job.
  #[serde(rename = "dirs_changed")]
  dirs_changed: i32,
  /// The number of directories deleted by this job.
  #[serde(rename = "dirs_deleted")]
  dirs_deleted: i32,
  /// The number of directories moved by this job.
  #[serde(rename = "dirs_moved")]
  dirs_moved: i32,
  /// The number of directories created by this job.
  #[serde(rename = "dirs_new")]
  dirs_new: i32,
  /// The amount of time in seconds between when the job was started and when it ended.  If the job has not yet ended, this is the amount of time since the job started.  This field is null if the job has not yet started.
  #[serde(rename = "duration")]
  duration: Option<i32>,
  /// The time the job ended in unix epoch seconds. The field is null if the job hasn't ended.
  #[serde(rename = "end_time")]
  end_time: Option<i32>,
  /// The primary error message for this job.
  #[serde(rename = "error")]
  error: String,
  /// The number of files with checksum errors skipped by this job.
  #[serde(rename = "error_checksum_files_skipped")]
  error_checksum_files_skipped: i32,
  /// The number of files with io errors skipped by this job.
  #[serde(rename = "error_io_files_skipped")]
  error_io_files_skipped: i32,
  /// The number of files with network errors skipped by this job.
  #[serde(rename = "error_net_files_skipped")]
  error_net_files_skipped: i32,
  /// A list of error messages for this job.
  #[serde(rename = "errors")]
  errors: Vec<String>,
  /// Tyhe number of data chunks that failed transmission.
  #[serde(rename = "failed_chunks")]
  failed_chunks: i32,
  /// The number of fifos replicated by this job.
  #[serde(rename = "fifos_replicated")]
  fifos_replicated: i32,
  /// The number of bytes transferred that belong to files.
  #[serde(rename = "file_data_bytes")]
  file_data_bytes: i32,
  /// The number of files changed by this job.
  #[serde(rename = "files_changed")]
  files_changed: i32,
  /// The number of files linked by this job.
  #[serde(rename = "files_linked")]
  files_linked: i32,
  /// The number of files created by this job.
  #[serde(rename = "files_new")]
  files_new: i32,
  /// The number of files selected by this job.
  #[serde(rename = "files_selected")]
  files_selected: i32,
  /// The number of files transferred by this job.
  #[serde(rename = "files_transferred")]
  files_transferred: i32,
  /// The number of files unlinked by this job.
  #[serde(rename = "files_unlinked")]
  files_unlinked: i32,
  /// The number of files with ads replicated by this job.
  #[serde(rename = "files_with_ads_replicated")]
  files_with_ads_replicated: i32,
  /// The number of LINs flipped by this job.
  #[serde(rename = "flipped_lins")]
  flipped_lins: i32,
  /// The number of hard links replicated by this job.
  #[serde(rename = "hard_links_replicated")]
  hard_links_replicated: i32,
  /// The number of hash exceptions fixed by this job.
  #[serde(rename = "hash_exceptions_fixed")]
  hash_exceptions_fixed: i32,
  /// The number of hash exceptions found by this job.
  #[serde(rename = "hash_exceptions_found")]
  hash_exceptions_found: i32,
  /// A unique identifier for this object.
  #[serde(rename = "id")]
  id: String,
  /// The ID of the job.
  #[serde(rename = "job_id")]
  job_id: Option<i32>,
  /// The number of LINs transferred by this job.
  #[serde(rename = "lins_total")]
  lins_total: i32,
  /// The total number of bytes sent to the source by this job.
  #[serde(rename = "network_bytes_to_source")]
  network_bytes_to_source: i32,
  /// The total number of bytes sent to the target by this job.
  #[serde(rename = "network_bytes_to_target")]
  network_bytes_to_target: i32,
  /// The number of new files replicated by this job.
  #[serde(rename = "new_files_replicated")]
  new_files_replicated: i32,
  /// The number of files that have been retransmitted by this job.
  #[serde(rename = "num_retransmitted_files")]
  num_retransmitted_files: i32,
  /// Data for each phase of this job.
  #[serde(rename = "phases")]
  phases: Vec<::models::SyncJobPhase>,
  /// 
  #[serde(rename = "policy")]
  policy: ::models::SyncReportPolicy,
  /// This is the action the policy is configured to perform.
  #[serde(rename = "policy_action")]
  policy_action: String,
  /// The ID of the policy.
  #[serde(rename = "policy_id")]
  policy_id: String,
  /// The name of the policy.
  #[serde(rename = "policy_name")]
  policy_name: String,
  /// The number of regular files replicated by this job.
  #[serde(rename = "regular_files_replicated")]
  regular_files_replicated: i32,
  /// The number of LINs resynched by this job.
  #[serde(rename = "resynced_lins")]
  resynced_lins: i32,
  /// The files that have been retransmitted by this job.
  #[serde(rename = "retransmitted_files")]
  retransmitted_files: Vec<String>,
  /// The number of times the job has been retried.
  #[serde(rename = "retry")]
  retry: i32,
  /// The number of data chunks currently being transmitted.
  #[serde(rename = "running_chunks")]
  running_chunks: i32,
  /// The number of sockets replicated by this job.
  #[serde(rename = "sockets_replicated")]
  sockets_replicated: i32,
  /// The number of bytes recovered on the source.
  #[serde(rename = "source_bytes_recovered")]
  source_bytes_recovered: i32,
  /// The number of directories created on the source.
  #[serde(rename = "source_directories_created")]
  source_directories_created: i32,
  /// The number of directories deleted on the source.
  #[serde(rename = "source_directories_deleted")]
  source_directories_deleted: i32,
  /// The number of directories linked on the source.
  #[serde(rename = "source_directories_linked")]
  source_directories_linked: i32,
  /// The number of directories unlinked on the source.
  #[serde(rename = "source_directories_unlinked")]
  source_directories_unlinked: i32,
  /// The number of directories visited on the source.
  #[serde(rename = "source_directories_visited")]
  source_directories_visited: i32,
  /// The number of files deleted on the source.
  #[serde(rename = "source_files_deleted")]
  source_files_deleted: i32,
  /// The number of files linked on the source.
  #[serde(rename = "source_files_linked")]
  source_files_linked: i32,
  /// The number of files unlinked on the source.
  #[serde(rename = "source_files_unlinked")]
  source_files_unlinked: i32,
  /// The number of sparse data bytes transferred by this job.
  #[serde(rename = "sparse_data_bytes")]
  sparse_data_bytes: i32,
  /// The time the job started in unix epoch seconds. The field is null if the job hasn't started.
  #[serde(rename = "start_time")]
  start_time: Option<i32>,
  /// The state of the job.
  #[serde(rename = "state")]
  state: String,
  /// The number of subreports that are available for this job report.
  #[serde(rename = "subreport_count")]
  subreport_count: i32,
  /// The number of data chunks that have been transmitted successfully.
  #[serde(rename = "succeeded_chunks")]
  succeeded_chunks: i32,
  /// The number of symlinks replicated by this job.
  #[serde(rename = "symlinks_replicated")]
  symlinks_replicated: i32,
  /// The type of sync being performed by this job.
  #[serde(rename = "sync_type")]
  sync_type: String,
  /// The number of bytes recovered on the target.
  #[serde(rename = "target_bytes_recovered")]
  target_bytes_recovered: i32,
  /// The number of directories created on the target.
  #[serde(rename = "target_directories_created")]
  target_directories_created: i32,
  /// The number of directories deleted on the target.
  #[serde(rename = "target_directories_deleted")]
  target_directories_deleted: i32,
  /// The number of directories linked on the target.
  #[serde(rename = "target_directories_linked")]
  target_directories_linked: i32,
  /// The number of directories unlinked on the target.
  #[serde(rename = "target_directories_unlinked")]
  target_directories_unlinked: i32,
  /// The number of files deleted on the target.
  #[serde(rename = "target_files_deleted")]
  target_files_deleted: i32,
  /// The number of files linked on the target.
  #[serde(rename = "target_files_linked")]
  target_files_linked: i32,
  /// The number of files unlinked on the target.
  #[serde(rename = "target_files_unlinked")]
  target_files_unlinked: i32,
  /// The target snapshots created by this job.
  #[serde(rename = "target_snapshots")]
  target_snapshots: Vec<String>,
  /// The total number of data chunks transmitted by this job.
  #[serde(rename = "total_chunks")]
  total_chunks: i32,
  /// The total number of bytes transferred by this job.
  #[serde(rename = "total_data_bytes")]
  total_data_bytes: i32,
  /// The number of files affected by this job.
  #[serde(rename = "total_files")]
  total_files: i32,
  /// The total number of bytes sent over the network by this job.
  #[serde(rename = "total_network_bytes")]
  total_network_bytes: i32,
  /// The total number of phases for this job.
  #[serde(rename = "total_phases")]
  total_phases: i32,
  /// The number of bytes unchanged by this job.
  #[serde(rename = "unchanged_data_bytes")]
  unchanged_data_bytes: i32,
  /// The number of up-to-date files skipped by this job.
  #[serde(rename = "up_to_date_files_skipped")]
  up_to_date_files_skipped: i32,
  /// The number of updated files replicated by this job.
  #[serde(rename = "updated_files_replicated")]
  updated_files_replicated: i32,
  /// The number of files with user conflicts skipped by this job.
  #[serde(rename = "user_conflict_files_skipped")]
  user_conflict_files_skipped: i32,
  /// A list of warning messages for this job.
  #[serde(rename = "warnings")]
  warnings: Vec<String>,
  /// The number of WORM committed files which needed to be reverted. Since WORM committed files cannot be reverted, this is the number of files that were preserved in the compliance store.
  #[serde(rename = "worm_committed_file_conflicts")]
  worm_committed_file_conflicts: i32
}

