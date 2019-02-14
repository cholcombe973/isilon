#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct TimezoneRegion {
    /// Clarifying comments on the region or timezone.
    #[serde(rename = "comments")]
    pub comments: Option<String>,
    /// A unique identifier for the timezone region.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The name of the region.
    #[serde(rename = "region")]
    pub region: Option<String>,
    /// A timezone.
    #[serde(rename = "timezone")]
    pub timezone: Option <crate::models::TimezoneRegionTimezone>,
}
