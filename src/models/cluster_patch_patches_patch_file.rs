

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterPatchPatchesPatchFile {
  /// The md5 checksum of the file.
  #[serde(rename = "md5")]
  md5: Option<String>,
  /// The path of the file.
  #[serde(rename = "path")]
  path: Option<String>
}

