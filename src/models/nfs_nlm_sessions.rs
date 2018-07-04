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
pub struct NfsNlmSessions {
  #[serde(rename = "sessions")]
  sessions: Option<Vec<::models::NfsNlmSessionsSession>>
}

impl NfsNlmSessions {
  pub fn new() -> NfsNlmSessions {
    NfsNlmSessions {
      sessions: None
    }
  }

  pub fn set_sessions(&mut self, sessions: Vec<::models::NfsNlmSessionsSession>) {
    self.sessions = Some(sessions);
  }

  pub fn with_sessions(mut self, sessions: Vec<::models::NfsNlmSessionsSession>) -> NfsNlmSessions {
    self.sessions = Some(sessions);
    self
  }

  pub fn sessions(&self) -> Option<&Vec<::models::NfsNlmSessionsSession>> {
    self.sessions.as_ref()
  }

  pub fn reset_sessions(&mut self) {
    self.sessions = None;
  }

}


