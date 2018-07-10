

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthAccessAccessItem {
  /// Specifies properties for access rights.
  #[serde(rename = "file")]
  file: Option<::models::AuthAccessAccessItemFile>,
  /// Specifies the ID of the user.
  #[serde(rename = "id")]
  id: Option<String>,
  /// Specifies the permissions that the user has on the share.
  #[serde(rename = "share")]
  share: Option<::models::AuthAccessAccessItemShare>,
  /// Specifies the persona for the user.
  #[serde(rename = "user")]
  user: Option<::models::AuthAccessAccessItemShareEffectiveUser>
}

