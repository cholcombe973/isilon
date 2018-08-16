#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultDirectoriesTotalUsage {
    /// Number of alternate data streams.
    #[serde(rename = "ads_cnt")]
    pub ads_cnt: i32,
    /// Number of directories.
    #[serde(rename = "dir_cnt")]
    pub dir_cnt: i32,
    /// Number of files.
    #[serde(rename = "file_cnt")]
    pub file_cnt: i32,
    /// Logical inode number.
    #[serde(rename = "lin")]
    pub lin: i32,
    /// Logical size directory in bytes.
    #[serde(rename = "log_size_sum")]
    pub log_size_sum: u64,
    /// Logical size sum of overflow in bytes.
    #[serde(rename = "log_size_sum_overflow")]
    pub log_size_sum_overflow: u64,
    /// Name of directory.
    #[serde(rename = "name")]
    pub name: String,
    /// Other count.
    #[serde(rename = "other_cnt")]
    pub other_cnt: i32,
    /// Parent directory inode.
    #[serde(rename = "parent")]
    pub parent: i32,
    /// Physical size directory in bytes.
    #[serde(rename = "phys_size_sum")]
    pub phys_size_sum: u64,
}
