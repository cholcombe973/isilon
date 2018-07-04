/* 
 * Isilon SDK
 *
 * Isilon SDK - Language bindings for the OneFS API
 *
 * OpenAPI spec version: 5
 * Contact: sdk@isilon.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NodesNodeFirmwareStatus : The firmware status for the node.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodesNodeFirmwareStatus {
  /// List of the firmware status for hardware components on the node.
  #[serde(rename = "devices")]
  devices: Option<Vec<::models::ClusterFirmwareStatusNodeDevice>>,
  /// Node is unavailable.
  #[serde(rename = "node_unavailable")]
  node_unavailable: Option<bool>,
  /// List of the firmware binary information for the installed firmware package.
  #[serde(rename = "package")]
  package: Option<Vec<::models::ClusterFirmwareStatusNodePackageItem>>
}

impl NodesNodeFirmwareStatus {
  /// The firmware status for the node.
  pub fn new() -> NodesNodeFirmwareStatus {
    NodesNodeFirmwareStatus {
      devices: None,
      node_unavailable: None,
      package: None
    }
  }

  pub fn set_devices(&mut self, devices: Vec<::models::ClusterFirmwareStatusNodeDevice>) {
    self.devices = Some(devices);
  }

  pub fn with_devices(mut self, devices: Vec<::models::ClusterFirmwareStatusNodeDevice>) -> NodesNodeFirmwareStatus {
    self.devices = Some(devices);
    self
  }

  pub fn devices(&self) -> Option<&Vec<::models::ClusterFirmwareStatusNodeDevice>> {
    self.devices.as_ref()
  }

  pub fn reset_devices(&mut self) {
    self.devices = None;
  }

  pub fn set_node_unavailable(&mut self, node_unavailable: bool) {
    self.node_unavailable = Some(node_unavailable);
  }

  pub fn with_node_unavailable(mut self, node_unavailable: bool) -> NodesNodeFirmwareStatus {
    self.node_unavailable = Some(node_unavailable);
    self
  }

  pub fn node_unavailable(&self) -> Option<&bool> {
    self.node_unavailable.as_ref()
  }

  pub fn reset_node_unavailable(&mut self) {
    self.node_unavailable = None;
  }

  pub fn set_package(&mut self, package: Vec<::models::ClusterFirmwareStatusNodePackageItem>) {
    self.package = Some(package);
  }

  pub fn with_package(mut self, package: Vec<::models::ClusterFirmwareStatusNodePackageItem>) -> NodesNodeFirmwareStatus {
    self.package = Some(package);
    self
  }

  pub fn package(&self) -> Option<&Vec<::models::ClusterFirmwareStatusNodePackageItem>> {
    self.package.as_ref()
  }

  pub fn reset_package(&mut self) {
    self.package = None;
  }

}


