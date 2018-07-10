

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultTopDirsDir {
  /// Directory access time
  #[serde(rename = "atime")]
  atime: i32,
  /// Directory creation begin time.
  #[serde(rename = "btime")]
  btime: i32,
  /// Unix inode change time.
  #[serde(rename = "ctime")]
  ctime: i32,
  /// Relative directory path under /ifs/.
  #[serde(rename = "path")]
  path: String
}

