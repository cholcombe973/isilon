/* 
 * Isilon SDK
 *
 * Isilon SDK - Language bindings for the OneFS API
 *
 * OpenAPI spec version: 5
 * Contact: sdk@isilon.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NtpServer : NTP server.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NtpServer {
  /// Key value from key_file that maps to this server.
  #[serde(rename = "key")]
  key: String
}

impl NtpServer {
  /// NTP server.
  pub fn new(key: String) -> NtpServer {
    NtpServer {
      key: key
    }
  }

  pub fn set_key(&mut self, key: String) {
    self.key = key;
  }

  pub fn with_key(mut self, key: String) -> NtpServer {
    self.key = key;
    self
  }

  pub fn key(&self) -> &String {
    &self.key
  }


}


