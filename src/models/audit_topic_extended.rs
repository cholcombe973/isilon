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
pub struct AuditTopicExtended {
  /// Specifies the system-provided ID for the audit topic.
  #[serde(rename = "id")]
  id: Option<String>,
  /// Specifies the maximum number of messages that can be sent and received at the same time. Messages that are sent and received at the same time can be lost if a system crash occurs. You can prevent message loss by setting this property to 0, which sets audit logs to synchronous.
  #[serde(rename = "max_cached_messages")]
  max_cached_messages: Option<i32>,
  /// Specifies the name of the audit topic.
  #[serde(rename = "name")]
  name: Option<String>
}

impl AuditTopicExtended {
  pub fn new() -> AuditTopicExtended {
    AuditTopicExtended {
      id: None,
      max_cached_messages: None,
      name: None
    }
  }

  pub fn set_id(&mut self, id: String) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: String) -> AuditTopicExtended {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_max_cached_messages(&mut self, max_cached_messages: i32) {
    self.max_cached_messages = Some(max_cached_messages);
  }

  pub fn with_max_cached_messages(mut self, max_cached_messages: i32) -> AuditTopicExtended {
    self.max_cached_messages = Some(max_cached_messages);
    self
  }

  pub fn max_cached_messages(&self) -> Option<&i32> {
    self.max_cached_messages.as_ref()
  }

  pub fn reset_max_cached_messages(&mut self) {
    self.max_cached_messages = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> AuditTopicExtended {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

}


