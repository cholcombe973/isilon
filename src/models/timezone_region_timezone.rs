#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct TimezoneRegionTimezone {
    /// The abbreviation for this timezone.
    #[serde(rename = "abbreviation")]
    pub abbreviation: Option<String>,
    /// The timezone path.  This is the unique identifier for the timezone.
    #[serde(rename = "path")]
    pub path: String,
}
