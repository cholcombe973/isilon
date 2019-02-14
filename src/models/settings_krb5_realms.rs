#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsKrb5Realms {
    #[serde(rename = "realm")]
    pub realm: Option<Vec <crate::models::SettingsKrb5RealmsRealmItem>>,
}
