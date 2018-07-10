

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FsaSettingsSettings {
  /// Name of question template to use for new FSA jobs.
  #[serde(rename = "default_template")]
  default_template: Option<String>,
  /// Maximum directory depth used for disk_usage question if not specified in the question.
  #[serde(rename = "disk_usage_depth")]
  disk_usage_depth: Option<i32>,
  /// Maximum age of non-pinned results in seconds.
  #[serde(rename = "max_age")]
  max_age: Option<i32>,
  /// Maximum number of non-pinned result sets to keep.
  #[serde(rename = "max_count")]
  max_count: Option<i32>,
  /// Squash depth to use for squash binning questions if not specified in the question.
  #[serde(rename = "squash_depth")]
  squash_depth: Option<i32>,
  /// Maximum number of items in a Top-N question result if not specified in the question.
  #[serde(rename = "top_n_max")]
  top_n_max: Option<i32>,
  /// If true, use a snapshot for consistency, otherwise analyze head.
  #[serde(rename = "use_snapshot")]
  use_snapshot: Option<bool>
}

