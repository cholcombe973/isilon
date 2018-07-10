

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct TimezoneRegion {
  /// Clarifying comments on the region or timezone.
  #[serde(rename = "comments")]
  comments: Option<String>,
  /// A unique identifier for the timezone region.
  #[serde(rename = "id")]
  id: Option<String>,
  /// The name of the region.
  #[serde(rename = "region")]
  region: Option<String>,
  /// A timezone.
  #[serde(rename = "timezone")]
  timezone: Option<::models::TimezoneRegionTimezone>
}

