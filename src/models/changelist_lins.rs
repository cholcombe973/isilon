

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangelistLins {
  /// 
  #[serde(rename = "atime")]
  atime: Option<::models::ChangelistLinsCtime>,
  /// 
  #[serde(rename = "ctime")]
  ctime: Option<::models::ChangelistLinsCtime>,
  /// The LIN number of the file associated with the entry.
  #[serde(rename = "id")]
  id: String,
  /// 
  #[serde(rename = "mtime")]
  mtime: Option<::models::ChangelistLinsCtime>,
  /// The path to the file associated with the entry.
  #[serde(rename = "path")]
  path: String,
  /// The size of the file associated with the entry.
  #[serde(rename = "size")]
  size: i32,
  /// Type of file.
  #[serde(rename = "type")]
  _type: String
}

