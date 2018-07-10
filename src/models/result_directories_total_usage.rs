

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultDirectoriesTotalUsage {
  /// Number of alternate data streams.
  #[serde(rename = "ads_cnt")]
  ads_cnt: i32,
  /// Number of directories.
  #[serde(rename = "dir_cnt")]
  dir_cnt: i32,
  /// Number of files.
  #[serde(rename = "file_cnt")]
  file_cnt: i32,
  /// Logical inode number.
  #[serde(rename = "lin")]
  lin: i32,
  /// Logical size directory in bytes.
  #[serde(rename = "log_size_sum")]
  log_size_sum: i32,
  /// Logical size sum of overflow in bytes.
  #[serde(rename = "log_size_sum_overflow")]
  log_size_sum_overflow: i32,
  /// Name of directory.
  #[serde(rename = "name")]
  name: String,
  /// Other count.
  #[serde(rename = "other_cnt")]
  other_cnt: i32,
  /// Parent directory inode.
  #[serde(rename = "parent")]
  parent: i32,
  /// Physical size directory in bytes.
  #[serde(rename = "phys_size_sum")]
  phys_size_sum: i32
}

