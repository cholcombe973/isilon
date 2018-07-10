

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragepoolTierExtended {
  /// The names or IDs of the tier's children.
  #[serde(rename = "children")]
  children: Option<Vec<String>>,
  /// The tier name.
  #[serde(rename = "name")]
  name: String,
  /// The system ID given to the tier.
  #[serde(rename = "id")]
  id: i32,
  /// The nodes that are part of this tier.
  #[serde(rename = "lnns")]
  lnns: Vec<i32>,
  /// Total pool usage.
  #[serde(rename = "usage")]
  usage: ::models::StoragepoolTierUsage
}

