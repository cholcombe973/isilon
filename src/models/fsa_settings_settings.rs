#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FsaSettingsSettings {
    /// Name of question template to use for new FSA jobs.
    #[serde(rename = "default_template")]
    pub default_template: Option<String>,
    /// Maximum directory depth used for disk_usage question if not specified in the question.
    #[serde(rename = "disk_usage_depth")]
    pub disk_usage_depth: Option<i32>,
    /// Maximum age of non-pinned results in seconds.
    #[serde(rename = "max_age")]
    pub max_age: Option<i32>,
    /// Maximum number of non-pinned result sets to keep.
    #[serde(rename = "max_count")]
    pub max_count: Option<i32>,
    /// Squash depth to use for squash binning questions if not specified in the question.
    #[serde(rename = "squash_depth")]
    pub squash_depth: Option<i32>,
    /// Maximum number of items in a Top-N question result if not specified in the question.
    #[serde(rename = "top_n_max")]
    pub top_n_max: Option<i32>,
    /// If true, use a snapshot for consistency, otherwise analyze head.
    #[serde(rename = "use_snapshot")]
    pub use_snapshot: Option<bool>,
}
