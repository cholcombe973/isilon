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
pub struct NdmpLogsNode {
  /// The page on this node's NDMP log file being returned.
  #[serde(rename = "current_page")]
  current_page: Option<String>,
  /// Error message, if the HTTP status returned from this node was not 200.
  #[serde(rename = "error")]
  error: Option<String>,
  /// Node ID of the node reporting this information.
  #[serde(rename = "id")]
  id: Option<i32>,
  /// Logical node number of the node reporting this information.
  #[serde(rename = "lnn")]
  lnn: Option<i32>,
  /// The log file entries from the current page on this node.
  #[serde(rename = "logs")]
  logs: Option<String>,
  /// The highest page number in this node's NDMP log file.
  #[serde(rename = "max_page")]
  max_page: Option<i32>,
  /// Status of the HTTP response from this node if not 200.  If 200, this field does not appear.
  #[serde(rename = "status")]
  status: Option<i32>
}

impl NdmpLogsNode {
  pub fn new() -> NdmpLogsNode {
    NdmpLogsNode {
      current_page: None,
      error: None,
      id: None,
      lnn: None,
      logs: None,
      max_page: None,
      status: None
    }
  }

  pub fn set_current_page(&mut self, current_page: String) {
    self.current_page = Some(current_page);
  }

  pub fn with_current_page(mut self, current_page: String) -> NdmpLogsNode {
    self.current_page = Some(current_page);
    self
  }

  pub fn current_page(&self) -> Option<&String> {
    self.current_page.as_ref()
  }

  pub fn reset_current_page(&mut self) {
    self.current_page = None;
  }

  pub fn set_error(&mut self, error: String) {
    self.error = Some(error);
  }

  pub fn with_error(mut self, error: String) -> NdmpLogsNode {
    self.error = Some(error);
    self
  }

  pub fn error(&self) -> Option<&String> {
    self.error.as_ref()
  }

  pub fn reset_error(&mut self) {
    self.error = None;
  }

  pub fn set_id(&mut self, id: i32) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i32) -> NdmpLogsNode {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&i32> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_lnn(&mut self, lnn: i32) {
    self.lnn = Some(lnn);
  }

  pub fn with_lnn(mut self, lnn: i32) -> NdmpLogsNode {
    self.lnn = Some(lnn);
    self
  }

  pub fn lnn(&self) -> Option<&i32> {
    self.lnn.as_ref()
  }

  pub fn reset_lnn(&mut self) {
    self.lnn = None;
  }

  pub fn set_logs(&mut self, logs: String) {
    self.logs = Some(logs);
  }

  pub fn with_logs(mut self, logs: String) -> NdmpLogsNode {
    self.logs = Some(logs);
    self
  }

  pub fn logs(&self) -> Option<&String> {
    self.logs.as_ref()
  }

  pub fn reset_logs(&mut self) {
    self.logs = None;
  }

  pub fn set_max_page(&mut self, max_page: i32) {
    self.max_page = Some(max_page);
  }

  pub fn with_max_page(mut self, max_page: i32) -> NdmpLogsNode {
    self.max_page = Some(max_page);
    self
  }

  pub fn max_page(&self) -> Option<&i32> {
    self.max_page.as_ref()
  }

  pub fn reset_max_page(&mut self) {
    self.max_page = None;
  }

  pub fn set_status(&mut self, status: i32) {
    self.status = Some(status);
  }

  pub fn with_status(mut self, status: i32) -> NdmpLogsNode {
    self.status = Some(status);
    self
  }

  pub fn status(&self) -> Option<&i32> {
    self.status.as_ref()
  }

  pub fn reset_status(&mut self) {
    self.status = None;
  }

}


