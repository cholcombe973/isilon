#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragepoolStatusUnhealthyItem {
    /// The affected nodes and/or drives.
    #[serde(rename = "affected")]
    pub affected: Vec<::models::StoragepoolStatusUnhealthyItemAffectedItem>,
    ///
    #[serde(rename = "diskpool")]
    pub diskpool: Option<::models::StoragepoolStatusUnhealthyItemDiskpool>,
    /// An array of containing any health issues with this pool.  If the pool is healthy, the list is empty.
    #[serde(rename = "health_flags")]
    pub health_flags: Option<Vec<String>>,
}
