

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthAccessAccessItemShare {
  /// Specifies the persona for the user.
  #[serde(rename = "effective_user")]
  effective_user: Option<::models::AuthAccessAccessItemShareEffectiveUser>,
  /// Specifies the permissions that the user has on the SHARE.
  #[serde(rename = "share_permissions")]
  share_permissions: Option<::models::AuthAccessAccessItemShareSharePermissions>
}

