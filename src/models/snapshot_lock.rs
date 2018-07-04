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
pub struct SnapshotLock {
  /// The Unix Epoch time the snapshot lock will expire and be eligible for automatic deletion.
  #[serde(rename = "expires")]
  expires: Option<i32>
}

impl SnapshotLock {
  pub fn new() -> SnapshotLock {
    SnapshotLock {
      expires: None
    }
  }

  pub fn set_expires(&mut self, expires: i32) {
    self.expires = Some(expires);
  }

  pub fn with_expires(mut self, expires: i32) -> SnapshotLock {
    self.expires = Some(expires);
    self
  }

  pub fn expires(&self) -> Option<&i32> {
    self.expires.as_ref()
  }

  pub fn reset_expires(&mut self) {
    self.expires = None;
  }

}


