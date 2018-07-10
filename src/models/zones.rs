#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Zones {
    #[serde(rename = "zones")]
    pub zones: Option<Vec<::models::ZoneExtended>>,
}
