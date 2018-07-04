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
pub struct AuthAccessAccessItemFile {
  /// Specifies absolute path in filesystem.
  #[serde(rename = "effective_path")]
  effective_path: Option<String>,
  /// Specifies the permissions that the user has on the file.
  #[serde(rename = "file_permissions")]
  file_permissions: Option<::models::AuthAccessAccessItemFileFilePermissions>,
  /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
  #[serde(rename = "group")]
  group: Option<::models::AuthAccessAccessItemFileGroup>,
  /// Specifies whether path is inside snapshot.
  #[serde(rename = "is_snapshot")]
  is_snapshot: Option<bool>,
  /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
  #[serde(rename = "owner")]
  owner: Option<::models::AuthAccessAccessItemFileGroup>
}

impl AuthAccessAccessItemFile {
  pub fn new() -> AuthAccessAccessItemFile {
    AuthAccessAccessItemFile {
      effective_path: None,
      file_permissions: None,
      group: None,
      is_snapshot: None,
      owner: None
    }
  }

  pub fn set_effective_path(&mut self, effective_path: String) {
    self.effective_path = Some(effective_path);
  }

  pub fn with_effective_path(mut self, effective_path: String) -> AuthAccessAccessItemFile {
    self.effective_path = Some(effective_path);
    self
  }

  pub fn effective_path(&self) -> Option<&String> {
    self.effective_path.as_ref()
  }

  pub fn reset_effective_path(&mut self) {
    self.effective_path = None;
  }

  pub fn set_file_permissions(&mut self, file_permissions: ::models::AuthAccessAccessItemFileFilePermissions) {
    self.file_permissions = Some(file_permissions);
  }

  pub fn with_file_permissions(mut self, file_permissions: ::models::AuthAccessAccessItemFileFilePermissions) -> AuthAccessAccessItemFile {
    self.file_permissions = Some(file_permissions);
    self
  }

  pub fn file_permissions(&self) -> Option<&::models::AuthAccessAccessItemFileFilePermissions> {
    self.file_permissions.as_ref()
  }

  pub fn reset_file_permissions(&mut self) {
    self.file_permissions = None;
  }

  pub fn set_group(&mut self, group: ::models::AuthAccessAccessItemFileGroup) {
    self.group = Some(group);
  }

  pub fn with_group(mut self, group: ::models::AuthAccessAccessItemFileGroup) -> AuthAccessAccessItemFile {
    self.group = Some(group);
    self
  }

  pub fn group(&self) -> Option<&::models::AuthAccessAccessItemFileGroup> {
    self.group.as_ref()
  }

  pub fn reset_group(&mut self) {
    self.group = None;
  }

  pub fn set_is_snapshot(&mut self, is_snapshot: bool) {
    self.is_snapshot = Some(is_snapshot);
  }

  pub fn with_is_snapshot(mut self, is_snapshot: bool) -> AuthAccessAccessItemFile {
    self.is_snapshot = Some(is_snapshot);
    self
  }

  pub fn is_snapshot(&self) -> Option<&bool> {
    self.is_snapshot.as_ref()
  }

  pub fn reset_is_snapshot(&mut self) {
    self.is_snapshot = None;
  }

  pub fn set_owner(&mut self, owner: ::models::AuthAccessAccessItemFileGroup) {
    self.owner = Some(owner);
  }

  pub fn with_owner(mut self, owner: ::models::AuthAccessAccessItemFileGroup) -> AuthAccessAccessItemFile {
    self.owner = Some(owner);
    self
  }

  pub fn owner(&self) -> Option<&::models::AuthAccessAccessItemFileGroup> {
    self.owner.as_ref()
  }

  pub fn reset_owner(&mut self) {
    self.owner = None;
  }

}


