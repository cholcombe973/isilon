/* 
 * Isilon SDK
 *
 * Isilon SDK - Language bindings for the OneFS API
 *
 * OpenAPI spec version: 5
 * Contact: sdk@isilon.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


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

impl AuthAccessAccessItem {
  pub fn new() -> AuthAccessAccessItem {
    AuthAccessAccessItem {
      file: None,
      id: None,
      share: None,
      user: None
    }
  }

  pub fn set_file(&mut self, file: ::models::AuthAccessAccessItemFile) {
    self.file = Some(file);
  }

  pub fn with_file(mut self, file: ::models::AuthAccessAccessItemFile) -> AuthAccessAccessItem {
    self.file = Some(file);
    self
  }

  pub fn file(&self) -> Option<&::models::AuthAccessAccessItemFile> {
    self.file.as_ref()
  }

  pub fn reset_file(&mut self) {
    self.file = None;
  }

  pub fn set_id(&mut self, id: String) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: String) -> AuthAccessAccessItem {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_share(&mut self, share: ::models::AuthAccessAccessItemShare) {
    self.share = Some(share);
  }

  pub fn with_share(mut self, share: ::models::AuthAccessAccessItemShare) -> AuthAccessAccessItem {
    self.share = Some(share);
    self
  }

  pub fn share(&self) -> Option<&::models::AuthAccessAccessItemShare> {
    self.share.as_ref()
  }

  pub fn reset_share(&mut self) {
    self.share = None;
  }

  pub fn set_user(&mut self, user: ::models::AuthAccessAccessItemShareEffectiveUser) {
    self.user = Some(user);
  }

  pub fn with_user(mut self, user: ::models::AuthAccessAccessItemShareEffectiveUser) -> AuthAccessAccessItem {
    self.user = Some(user);
    self
  }

  pub fn user(&self) -> Option<&::models::AuthAccessAccessItemShareEffectiveUser> {
    self.user.as_ref()
  }

  pub fn reset_user(&mut self) {
    self.user = None;
  }

}


