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
pub struct NdmpContextsBackupSession {
  /// bre context id; This is not applicable to restore sessions.
  #[serde(rename = "bre_context_id")]
  bre_context_id: Option<String>,
  /// Session ID
  #[serde(rename = "session_id")]
  session_id: Option<String>,
  /// Session creation time
  #[serde(rename = "start_time")]
  start_time: Option<i32>,
  /// The status of the session
  #[serde(rename = "status")]
  status: Option<String>,
  /// Stream ID
  #[serde(rename = "stream_id")]
  stream_id: Option<String>
}

impl NdmpContextsBackupSession {
  pub fn new() -> NdmpContextsBackupSession {
    NdmpContextsBackupSession {
      bre_context_id: None,
      session_id: None,
      start_time: None,
      status: None,
      stream_id: None
    }
  }

  pub fn set_bre_context_id(&mut self, bre_context_id: String) {
    self.bre_context_id = Some(bre_context_id);
  }

  pub fn with_bre_context_id(mut self, bre_context_id: String) -> NdmpContextsBackupSession {
    self.bre_context_id = Some(bre_context_id);
    self
  }

  pub fn bre_context_id(&self) -> Option<&String> {
    self.bre_context_id.as_ref()
  }

  pub fn reset_bre_context_id(&mut self) {
    self.bre_context_id = None;
  }

  pub fn set_session_id(&mut self, session_id: String) {
    self.session_id = Some(session_id);
  }

  pub fn with_session_id(mut self, session_id: String) -> NdmpContextsBackupSession {
    self.session_id = Some(session_id);
    self
  }

  pub fn session_id(&self) -> Option<&String> {
    self.session_id.as_ref()
  }

  pub fn reset_session_id(&mut self) {
    self.session_id = None;
  }

  pub fn set_start_time(&mut self, start_time: i32) {
    self.start_time = Some(start_time);
  }

  pub fn with_start_time(mut self, start_time: i32) -> NdmpContextsBackupSession {
    self.start_time = Some(start_time);
    self
  }

  pub fn start_time(&self) -> Option<&i32> {
    self.start_time.as_ref()
  }

  pub fn reset_start_time(&mut self) {
    self.start_time = None;
  }

  pub fn set_status(&mut self, status: String) {
    self.status = Some(status);
  }

  pub fn with_status(mut self, status: String) -> NdmpContextsBackupSession {
    self.status = Some(status);
    self
  }

  pub fn status(&self) -> Option<&String> {
    self.status.as_ref()
  }

  pub fn reset_status(&mut self) {
    self.status = None;
  }

  pub fn set_stream_id(&mut self, stream_id: String) {
    self.stream_id = Some(stream_id);
  }

  pub fn with_stream_id(mut self, stream_id: String) -> NdmpContextsBackupSession {
    self.stream_id = Some(stream_id);
    self
  }

  pub fn stream_id(&self) -> Option<&String> {
    self.stream_id.as_ref()
  }

  pub fn reset_stream_id(&mut self) {
    self.stream_id = None;
  }

}


