

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SmbOpenfile {
  /// Path of file within /ifs.
  #[serde(rename = "file")]
  file: String,
  /// The file ID.
  #[serde(rename = "id")]
  id: i32,
  /// Number of locks user holds on file.
  #[serde(rename = "locks")]
  locks: i32,
  /// The user's permissions on file.
  #[serde(rename = "permissions")]
  permissions: Vec<String>,
  /// User holding file open.
  #[serde(rename = "user")]
  user: String
}

