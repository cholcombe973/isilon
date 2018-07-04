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
pub struct SettingsMappingExtendedExtended {
  /// The FQDN of the source domain to map.
  #[serde(rename = "domain")]
  domain: String,
  #[serde(rename = "id")]
  id: Option<String>,
  /// The FQDN of destination domain to map to.
  #[serde(rename = "mapping")]
  mapping: String,
  /// The authentication provider type.
  #[serde(rename = "type")]
  _type: String
}

impl SettingsMappingExtendedExtended {
  pub fn new(domain: String, mapping: String, _type: String) -> SettingsMappingExtendedExtended {
    SettingsMappingExtendedExtended {
      domain: domain,
      id: None,
      mapping: mapping,
      _type: _type
    }
  }

  pub fn set_domain(&mut self, domain: String) {
    self.domain = domain;
  }

  pub fn with_domain(mut self, domain: String) -> SettingsMappingExtendedExtended {
    self.domain = domain;
    self
  }

  pub fn domain(&self) -> &String {
    &self.domain
  }


  pub fn set_id(&mut self, id: String) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: String) -> SettingsMappingExtendedExtended {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_mapping(&mut self, mapping: String) {
    self.mapping = mapping;
  }

  pub fn with_mapping(mut self, mapping: String) -> SettingsMappingExtendedExtended {
    self.mapping = mapping;
    self
  }

  pub fn mapping(&self) -> &String {
    &self.mapping
  }


  pub fn set__type(&mut self, _type: String) {
    self._type = _type;
  }

  pub fn with__type(mut self, _type: String) -> SettingsMappingExtendedExtended {
    self._type = _type;
    self
  }

  pub fn _type(&self) -> &String {
    &self._type
  }


}


