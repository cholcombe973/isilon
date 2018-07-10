

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragepoolStatusUnhealthyItem {
  /// The affected nodes and/or drives.
  #[serde(rename = "affected")]
  affected: Vec<::models::StoragepoolStatusUnhealthyItemAffectedItem>,
  /// 
  #[serde(rename = "diskpool")]
  diskpool: Option<::models::StoragepoolStatusUnhealthyItemDiskpool>,
  /// An array of containing any health issues with this pool.  If the pool is healthy, the list is empty.
  #[serde(rename = "health_flags")]
  health_flags: Option<Vec<String>>
}

