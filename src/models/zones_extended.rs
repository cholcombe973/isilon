#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ZonesExtended {
    #[serde(rename = "zones")]
    pub zones: Option<Vec<::models::ZoneExtendedExtended>>,
}
