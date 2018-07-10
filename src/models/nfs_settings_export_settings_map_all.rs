

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NfsSettingsExportSettingsMapAll {
  /// True if the user mapping is applied.
  #[serde(rename = "enabled")]
  enabled: Option<bool>,
  /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
  #[serde(rename = "primary_group")]
  primary_group: Option<::models::AuthAccessAccessItemFileGroup>,
  /// Specifies persona properties for the secondary user group. A persona consists of either a type and name, or an ID.
  #[serde(rename = "secondary_groups")]
  secondary_groups: Option<Vec<::models::NfsSettingsExportSettingsMapAllSecondaryGroups>>,
  /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
  #[serde(rename = "user")]
  user: Option<::models::AuthAccessAccessItemFileGroup>
}

