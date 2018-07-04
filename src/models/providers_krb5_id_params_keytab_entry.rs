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
pub struct ProvidersKrb5IdParamsKeytabEntry {
  /// Specifies the encryption types.
  #[serde(rename = "encryption")]
  encryption: Option<Vec<String>>,
  /// Specifies the version number of the SPN key.
  #[serde(rename = "kvno")]
  kvno: Option<i32>,
  /// Specifies the Service Principal Name (SPN).
  #[serde(rename = "spn")]
  spn: Option<String>,
  /// Specifies the time the SPN key was created.
  #[serde(rename = "timestamp")]
  timestamp: Option<i32>
}

impl ProvidersKrb5IdParamsKeytabEntry {
  pub fn new() -> ProvidersKrb5IdParamsKeytabEntry {
    ProvidersKrb5IdParamsKeytabEntry {
      encryption: None,
      kvno: None,
      spn: None,
      timestamp: None
    }
  }

  pub fn set_encryption(&mut self, encryption: Vec<String>) {
    self.encryption = Some(encryption);
  }

  pub fn with_encryption(mut self, encryption: Vec<String>) -> ProvidersKrb5IdParamsKeytabEntry {
    self.encryption = Some(encryption);
    self
  }

  pub fn encryption(&self) -> Option<&Vec<String>> {
    self.encryption.as_ref()
  }

  pub fn reset_encryption(&mut self) {
    self.encryption = None;
  }

  pub fn set_kvno(&mut self, kvno: i32) {
    self.kvno = Some(kvno);
  }

  pub fn with_kvno(mut self, kvno: i32) -> ProvidersKrb5IdParamsKeytabEntry {
    self.kvno = Some(kvno);
    self
  }

  pub fn kvno(&self) -> Option<&i32> {
    self.kvno.as_ref()
  }

  pub fn reset_kvno(&mut self) {
    self.kvno = None;
  }

  pub fn set_spn(&mut self, spn: String) {
    self.spn = Some(spn);
  }

  pub fn with_spn(mut self, spn: String) -> ProvidersKrb5IdParamsKeytabEntry {
    self.spn = Some(spn);
    self
  }

  pub fn spn(&self) -> Option<&String> {
    self.spn.as_ref()
  }

  pub fn reset_spn(&mut self) {
    self.spn = None;
  }

  pub fn set_timestamp(&mut self, timestamp: i32) {
    self.timestamp = Some(timestamp);
  }

  pub fn with_timestamp(mut self, timestamp: i32) -> ProvidersKrb5IdParamsKeytabEntry {
    self.timestamp = Some(timestamp);
    self
  }

  pub fn timestamp(&self) -> Option<&i32> {
    self.timestamp.as_ref()
  }

  pub fn reset_timestamp(&mut self) {
    self.timestamp = None;
  }

}


