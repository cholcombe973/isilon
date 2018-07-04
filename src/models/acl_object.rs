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
pub struct AclObject {
  #[serde(rename = "accessrights")]
  accessrights: Option<Vec<String>>,
  #[serde(rename = "accesstype")]
  accesstype: Option<String>,
  #[serde(rename = "inherit_flags")]
  inherit_flags: Option<Vec<bool>>,
  #[serde(rename = "op")]
  op: Option<String>,
  #[serde(rename = "trustee")]
  trustee: Option<::models::MemberObject>,
}

impl AclObject {
  pub fn new() -> AclObject {
    AclObject {
      accessrights: None,
      accesstype: None,
      inherit_flags: None,
      op: None,
      trustee: None,
    }
  }

  pub fn set_accessrights(&mut self, accessrights: Vec<String>) {
    self.accessrights = Some(accessrights);
  }

  pub fn with_accessrights(mut self, accessrights: Vec<String>) -> AclObject {
    self.accessrights = Some(accessrights);
    self
  }

  pub fn accessrights(&self) -> Option<&Vec<String>> {
    self.accessrights.as_ref()
  }

  pub fn reset_accessrights(&mut self) {
    self.accessrights = None;
  }

  pub fn set_accesstype(&mut self, accesstype: String) {
    self.accesstype = Some(accesstype);
  }

  pub fn with_accesstype(mut self, accesstype: String) -> AclObject {
    self.accesstype = Some(accesstype);
    self
  }

  pub fn accesstype(&self) -> Option<&String> {
    self.accesstype.as_ref()
  }

  pub fn reset_accesstype(&mut self) {
    self.accesstype = None;
  }

  pub fn set_inherit_flags(&mut self, inherit_flags: Vec<bool>) {
    self.inherit_flags = Some(inherit_flags);
  }

  pub fn with_inherit_flags(mut self, inherit_flags: Vec<bool>) -> AclObject {
    self.inherit_flags = Some(inherit_flags);
    self
  }

  pub fn inherit_flags(&self) -> Option<&Vec<bool>> {
    self.inherit_flags.as_ref()
  }

  pub fn reset_inherit_flags(&mut self) {
    self.inherit_flags = None;
  }

  pub fn set_op(&mut self, op: String) {
    self.op = Some(op);
  }

  pub fn with_op(mut self, op: String) -> AclObject {
    self.op = Some(op);
    self
  }

  pub fn op(&self) -> Option<&String> {
    self.op.as_ref()
  }

  pub fn reset_op(&mut self) {
    self.op = None;
  }

  pub fn set_trustee(&mut self, trustee: ::models::MemberObject) {
    self.trustee = Some(trustee);
  }

  pub fn with_trustee(mut self, trustee: ::models::MemberObject) -> AclObject {
    self.trustee = Some(trustee);
    self
  }

  pub fn trustee(&self) -> Option<&::models::MemberObject> {
    self.trustee.as_ref()
  }

  pub fn reset_trustee(&mut self) {
    self.trustee = None;
  }
}