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
pub struct CompatibilitiesClassActiveItem {
  /// Do not create compatibility, only assess if creation is possible.
  #[serde(rename = "assess")]
  assess: Option<bool>,
  /// The first class in the desired compatibility
  #[serde(rename = "class_1")]
  class_1: String,
  /// The second class in the desired compatibility
  #[serde(rename = "class_2")]
  class_2: String
}

impl CompatibilitiesClassActiveItem {
  pub fn new(class_1: String, class_2: String) -> CompatibilitiesClassActiveItem {
    CompatibilitiesClassActiveItem {
      assess: None,
      class_1: class_1,
      class_2: class_2
    }
  }

  pub fn set_assess(&mut self, assess: bool) {
    self.assess = Some(assess);
  }

  pub fn with_assess(mut self, assess: bool) -> CompatibilitiesClassActiveItem {
    self.assess = Some(assess);
    self
  }

  pub fn assess(&self) -> Option<&bool> {
    self.assess.as_ref()
  }

  pub fn reset_assess(&mut self) {
    self.assess = None;
  }

  pub fn set_class_1(&mut self, class_1: String) {
    self.class_1 = class_1;
  }

  pub fn with_class_1(mut self, class_1: String) -> CompatibilitiesClassActiveItem {
    self.class_1 = class_1;
    self
  }

  pub fn class_1(&self) -> &String {
    &self.class_1
  }


  pub fn set_class_2(&mut self, class_2: String) {
    self.class_2 = class_2;
  }

  pub fn with_class_2(mut self, class_2: String) -> CompatibilitiesClassActiveItem {
    self.class_2 = class_2;
    self
  }

  pub fn class_2(&self) -> &String {
    &self.class_2
  }


}


