

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragepoolTier {
  /// The names or IDs of the tier's children.
  #[serde(rename = "children")]
  children: Option<Vec<String>>,
  /// The tier name.
  #[serde(rename = "name")]
  name: Option<String>
}

