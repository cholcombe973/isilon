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
pub struct AuthUsers {
  #[serde(rename = "users")]
  users: Option<Vec<::models::MappingUsersLookupMappingItemUser>>
}

impl AuthUsers {
  pub fn new() -> AuthUsers {
    AuthUsers {
      users: None
    }
  }

  pub fn set_users(&mut self, users: Vec<::models::MappingUsersLookupMappingItemUser>) {
    self.users = Some(users);
  }

  pub fn with_users(mut self, users: Vec<::models::MappingUsersLookupMappingItemUser>) -> AuthUsers {
    self.users = Some(users);
    self
  }

  pub fn users(&self) -> Option<&Vec<::models::MappingUsersLookupMappingItemUser>> {
    self.users.as_ref()
  }

  pub fn reset_users(&mut self) {
    self.users = None;
  }

}


