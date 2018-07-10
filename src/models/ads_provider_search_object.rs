

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AdsProviderSearchObject {
  #[serde(rename = "description")]
  description: Option<String>,
  #[serde(rename = "display_name")]
  display_name: Option<String>,
  /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
  #[serde(rename = "id")]
  id: Option<::models::AuthAccessAccessItemFileGroup>
}

