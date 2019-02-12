#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct TimezoneSettings {
    /// A timezone.
    #[serde(rename = "settings")]
    pub settings: Option <crate::models::TimezoneRegionTimezone>,
}
