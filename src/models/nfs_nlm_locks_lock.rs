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
pub struct NfsNlmLocksLock {
  /// Specifies the client host name and IP address.
  #[serde(rename = "client")]
  client: Option<String>,
  /// Specifies the client ID.
  #[serde(rename = "client_id")]
  client_id: Option<String>,
  /// Specifies the UNIX EPoch time that the lock was created.
  #[serde(rename = "created")]
  created: Option<i32>,
  /// Specifies the system-assigned ID given to the lock. This value is returned when the lock is created with the POST method.
  #[serde(rename = "id")]
  id: Option<String>,
  /// Specifies the LIN in /ifs that is locked.
  #[serde(rename = "lin")]
  lin: Option<String>,
  /// Specifies the lock type.
  #[serde(rename = "lock_type")]
  lock_type: Option<String>,
  /// Specifies the path under /ifs that is locked.
  #[serde(rename = "path")]
  path: Option<String>,
  /// Specifies the byte range within the locked file.
  #[serde(rename = "range")]
  range: Option<Vec<i32>>
}

impl NfsNlmLocksLock {
  pub fn new() -> NfsNlmLocksLock {
    NfsNlmLocksLock {
      client: None,
      client_id: None,
      created: None,
      id: None,
      lin: None,
      lock_type: None,
      path: None,
      range: None
    }
  }

  pub fn set_client(&mut self, client: String) {
    self.client = Some(client);
  }

  pub fn with_client(mut self, client: String) -> NfsNlmLocksLock {
    self.client = Some(client);
    self
  }

  pub fn client(&self) -> Option<&String> {
    self.client.as_ref()
  }

  pub fn reset_client(&mut self) {
    self.client = None;
  }

  pub fn set_client_id(&mut self, client_id: String) {
    self.client_id = Some(client_id);
  }

  pub fn with_client_id(mut self, client_id: String) -> NfsNlmLocksLock {
    self.client_id = Some(client_id);
    self
  }

  pub fn client_id(&self) -> Option<&String> {
    self.client_id.as_ref()
  }

  pub fn reset_client_id(&mut self) {
    self.client_id = None;
  }

  pub fn set_created(&mut self, created: i32) {
    self.created = Some(created);
  }

  pub fn with_created(mut self, created: i32) -> NfsNlmLocksLock {
    self.created = Some(created);
    self
  }

  pub fn created(&self) -> Option<&i32> {
    self.created.as_ref()
  }

  pub fn reset_created(&mut self) {
    self.created = None;
  }

  pub fn set_id(&mut self, id: String) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: String) -> NfsNlmLocksLock {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_lin(&mut self, lin: String) {
    self.lin = Some(lin);
  }

  pub fn with_lin(mut self, lin: String) -> NfsNlmLocksLock {
    self.lin = Some(lin);
    self
  }

  pub fn lin(&self) -> Option<&String> {
    self.lin.as_ref()
  }

  pub fn reset_lin(&mut self) {
    self.lin = None;
  }

  pub fn set_lock_type(&mut self, lock_type: String) {
    self.lock_type = Some(lock_type);
  }

  pub fn with_lock_type(mut self, lock_type: String) -> NfsNlmLocksLock {
    self.lock_type = Some(lock_type);
    self
  }

  pub fn lock_type(&self) -> Option<&String> {
    self.lock_type.as_ref()
  }

  pub fn reset_lock_type(&mut self) {
    self.lock_type = None;
  }

  pub fn set_path(&mut self, path: String) {
    self.path = Some(path);
  }

  pub fn with_path(mut self, path: String) -> NfsNlmLocksLock {
    self.path = Some(path);
    self
  }

  pub fn path(&self) -> Option<&String> {
    self.path.as_ref()
  }

  pub fn reset_path(&mut self) {
    self.path = None;
  }

  pub fn set_range(&mut self, range: Vec<i32>) {
    self.range = Some(range);
  }

  pub fn with_range(mut self, range: Vec<i32>) -> NfsNlmLocksLock {
    self.range = Some(range);
    self
  }

  pub fn range(&self) -> Option<&Vec<i32>> {
    self.range.as_ref()
  }

  pub fn reset_range(&mut self) {
    self.range = None;
  }

}


