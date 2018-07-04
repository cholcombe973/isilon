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
pub struct AccessPointCreateParams {
  /// Absolute file system path of access point.
  #[serde(rename = "path")]
  path: String
}

impl AccessPointCreateParams {
  pub fn new(path: String) -> AccessPointCreateParams {
    AccessPointCreateParams {
      path: path
    }
  }

  pub fn set_path(&mut self, path: String) {
    self.path = path;
  }

  pub fn with_path(mut self, path: String) -> AccessPointCreateParams {
    self.path = path;
    self
  }

  pub fn path(&self) -> &String {
    &self.path
  }


}


