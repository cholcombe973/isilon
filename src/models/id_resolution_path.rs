

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IdResolutionPath {
  /// Logical Inode Number (LIN). A 64-bit number which uniquely identifies a file throughout its life.
  #[serde(rename = "lin")]
  lin: Option<String>,
  /// The full path associated with the lin. null if the lin cannot be resolved to a path.
  #[serde(rename = "path")]
  path: Option<String>
}

