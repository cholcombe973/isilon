#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NfsSettingsExportSettingsMapAll {
    /// True if the user mapping is applied.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
    #[serde(rename = "primary_group")]
    pub primary_group: Option <crate::models::AuthAccessAccessItemFileGroup>,
    /// Specifies persona properties for the secondary user group. A persona consists of either a type and name, or an ID.
    #[serde(rename = "secondary_groups")]
    pub secondary_groups: Option<Vec <crate::models::NfsSettingsExportSettingsMapAllSecondaryGroups>>,
    /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
    #[serde(rename = "user")]
    pub user: Option <crate::models::AuthAccessAccessItemFileGroup>,
}
