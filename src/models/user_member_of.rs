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
pub struct UserMemberOf {
  #[serde(rename = "member_of")]
  member_of: Option<Vec<::models::AuthAccessAccessItemFileGroup>>,
  /// Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options).
  #[serde(rename = "resume")]
  resume: Option<String>
}

impl UserMemberOf {
  pub fn new() -> UserMemberOf {
    UserMemberOf {
      member_of: None,
      resume: None
    }
  }

  pub fn set_member_of(&mut self, member_of: Vec<::models::AuthAccessAccessItemFileGroup>) {
    self.member_of = Some(member_of);
  }

  pub fn with_member_of(mut self, member_of: Vec<::models::AuthAccessAccessItemFileGroup>) -> UserMemberOf {
    self.member_of = Some(member_of);
    self
  }

  pub fn member_of(&self) -> Option<&Vec<::models::AuthAccessAccessItemFileGroup>> {
    self.member_of.as_ref()
  }

  pub fn reset_member_of(&mut self) {
    self.member_of = None;
  }

  pub fn set_resume(&mut self, resume: String) {
    self.resume = Some(resume);
  }

  pub fn with_resume(mut self, resume: String) -> UserMemberOf {
    self.resume = Some(resume);
    self
  }

  pub fn resume(&self) -> Option<&String> {
    self.resume.as_ref()
  }

  pub fn reset_resume(&mut self) {
    self.resume = None;
  }

}


