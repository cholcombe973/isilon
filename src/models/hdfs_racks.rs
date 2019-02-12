#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HdfsRacks {
    #[serde(rename = "racks")]
    pub racks: Option<Vec <crate::models::HdfsRackExtended>>,
}
