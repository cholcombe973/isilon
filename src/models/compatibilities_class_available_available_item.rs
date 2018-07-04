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
pub struct CompatibilitiesClassAvailableAvailableItem {
  /// The first class in an available compatibility
  #[serde(rename = "class_1")]
  class_1: String,
  /// The second class in an available compatibility
  #[serde(rename = "class_2")]
  class_2: String
}

impl CompatibilitiesClassAvailableAvailableItem {
  pub fn new(class_1: String, class_2: String) -> CompatibilitiesClassAvailableAvailableItem {
    CompatibilitiesClassAvailableAvailableItem {
      class_1: class_1,
      class_2: class_2
    }
  }

  pub fn set_class_1(&mut self, class_1: String) {
    self.class_1 = class_1;
  }

  pub fn with_class_1(mut self, class_1: String) -> CompatibilitiesClassAvailableAvailableItem {
    self.class_1 = class_1;
    self
  }

  pub fn class_1(&self) -> &String {
    &self.class_1
  }


  pub fn set_class_2(&mut self, class_2: String) {
    self.class_2 = class_2;
  }

  pub fn with_class_2(mut self, class_2: String) -> CompatibilitiesClassAvailableAvailableItem {
    self.class_2 = class_2;
    self
  }

  pub fn class_2(&self) -> &String {
    &self.class_2
  }


}


