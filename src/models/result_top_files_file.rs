

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultTopFilesFile {
  /// File access time.
  #[serde(rename = "atime")]
  atime: i32,
  /// File creation begin time.
  #[serde(rename = "btime")]
  btime: i32,
  /// Unix inode change time.
  #[serde(rename = "ctime")]
  ctime: i32,
  /// Logical file size in bytes.
  #[serde(rename = "log_size")]
  log_size: i32,
  /// Relative file path under /ifs/.
  #[serde(rename = "path")]
  path: String,
  /// Physical file size in bytes.
  #[serde(rename = "phys_size")]
  phys_size: i32
}

