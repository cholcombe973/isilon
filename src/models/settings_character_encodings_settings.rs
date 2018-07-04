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
pub struct SettingsCharacterEncodingsSettings {
  /// Current character encoding.
  #[serde(rename = "current-encoding")]
  current_encoding: String,
  /// Default character encoding.
  #[serde(rename = "default-encoding")]
  default_encoding: String,
  /// A list of supported encoding values.
  #[serde(rename = "encodings")]
  encodings: Vec<String>
}

impl SettingsCharacterEncodingsSettings {
  pub fn new(current_encoding: String, default_encoding: String, encodings: Vec<String>) -> SettingsCharacterEncodingsSettings {
    SettingsCharacterEncodingsSettings {
      current_encoding: current_encoding,
      default_encoding: default_encoding,
      encodings: encodings
    }
  }

  pub fn set_current_encoding(&mut self, current_encoding: String) {
    self.current_encoding = current_encoding;
  }

  pub fn with_current_encoding(mut self, current_encoding: String) -> SettingsCharacterEncodingsSettings {
    self.current_encoding = current_encoding;
    self
  }

  pub fn current_encoding(&self) -> &String {
    &self.current_encoding
  }


  pub fn set_default_encoding(&mut self, default_encoding: String) {
    self.default_encoding = default_encoding;
  }

  pub fn with_default_encoding(mut self, default_encoding: String) -> SettingsCharacterEncodingsSettings {
    self.default_encoding = default_encoding;
    self
  }

  pub fn default_encoding(&self) -> &String {
    &self.default_encoding
  }


  pub fn set_encodings(&mut self, encodings: Vec<String>) {
    self.encodings = encodings;
  }

  pub fn with_encodings(mut self, encodings: Vec<String>) -> SettingsCharacterEncodingsSettings {
    self.encodings = encodings;
    self
  }

  pub fn encodings(&self) -> &Vec<String> {
    &self.encodings
  }


}


