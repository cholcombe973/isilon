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
pub struct EventChannelExtended {
  /// Nodes (LNNs) that can be masters for this channel
  #[serde(rename = "allowed_nodes")]
  allowed_nodes: Option<Vec<i32>>,
  /// Channel is to be used or not
  #[serde(rename = "enabled")]
  enabled: Option<bool>,
  /// Nodes (LNNs) that can NOT be the masters for this channel
  #[serde(rename = "excluded_nodes")]
  excluded_nodes: Option<Vec<i32>>,
  /// Parameters to be used for an smtp channel
  #[serde(rename = "parameters")]
  parameters: Option<::models::EventChannelParameters>,
  /// Channel is a pre-defined system channel
  #[serde(rename = "system")]
  system: Option<bool>,
  /// The mechanism used by the channel
  #[serde(rename = "type")]
  _type: Option<String>,
  /// Unique identifier.
  #[serde(rename = "id")]
  id: Option<i32>,
  /// Channel name,  may not contain /, max length 254.
  #[serde(rename = "name")]
  name: Option<String>
}

impl EventChannelExtended {
  pub fn new() -> EventChannelExtended {
    EventChannelExtended {
      allowed_nodes: None,
      enabled: None,
      excluded_nodes: None,
      parameters: None,
      system: None,
      _type: None,
      id: None,
      name: None
    }
  }

  pub fn set_allowed_nodes(&mut self, allowed_nodes: Vec<i32>) {
    self.allowed_nodes = Some(allowed_nodes);
  }

  pub fn with_allowed_nodes(mut self, allowed_nodes: Vec<i32>) -> EventChannelExtended {
    self.allowed_nodes = Some(allowed_nodes);
    self
  }

  pub fn allowed_nodes(&self) -> Option<&Vec<i32>> {
    self.allowed_nodes.as_ref()
  }

  pub fn reset_allowed_nodes(&mut self) {
    self.allowed_nodes = None;
  }

  pub fn set_enabled(&mut self, enabled: bool) {
    self.enabled = Some(enabled);
  }

  pub fn with_enabled(mut self, enabled: bool) -> EventChannelExtended {
    self.enabled = Some(enabled);
    self
  }

  pub fn enabled(&self) -> Option<&bool> {
    self.enabled.as_ref()
  }

  pub fn reset_enabled(&mut self) {
    self.enabled = None;
  }

  pub fn set_excluded_nodes(&mut self, excluded_nodes: Vec<i32>) {
    self.excluded_nodes = Some(excluded_nodes);
  }

  pub fn with_excluded_nodes(mut self, excluded_nodes: Vec<i32>) -> EventChannelExtended {
    self.excluded_nodes = Some(excluded_nodes);
    self
  }

  pub fn excluded_nodes(&self) -> Option<&Vec<i32>> {
    self.excluded_nodes.as_ref()
  }

  pub fn reset_excluded_nodes(&mut self) {
    self.excluded_nodes = None;
  }

  pub fn set_parameters(&mut self, parameters: ::models::EventChannelParameters) {
    self.parameters = Some(parameters);
  }

  pub fn with_parameters(mut self, parameters: ::models::EventChannelParameters) -> EventChannelExtended {
    self.parameters = Some(parameters);
    self
  }

  pub fn parameters(&self) -> Option<&::models::EventChannelParameters> {
    self.parameters.as_ref()
  }

  pub fn reset_parameters(&mut self) {
    self.parameters = None;
  }

  pub fn set_system(&mut self, system: bool) {
    self.system = Some(system);
  }

  pub fn with_system(mut self, system: bool) -> EventChannelExtended {
    self.system = Some(system);
    self
  }

  pub fn system(&self) -> Option<&bool> {
    self.system.as_ref()
  }

  pub fn reset_system(&mut self) {
    self.system = None;
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: String) -> EventChannelExtended {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&String> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

  pub fn set_id(&mut self, id: i32) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: i32) -> EventChannelExtended {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&i32> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> EventChannelExtended {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

}


