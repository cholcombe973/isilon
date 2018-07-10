

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterConfigTimezone {
  /// Timezone abbreviation.
  #[serde(rename = "abbreviation")]
  abbreviation: Option<String>,
  /// Customer timezone information.
  #[serde(rename = "custom")]
  custom: Option<String>,
  /// Timezone full name.
  #[serde(rename = "name")]
  name: Option<String>,
  /// Timezone hierarchical name.
  #[serde(rename = "path")]
  path: Option<String>
}

