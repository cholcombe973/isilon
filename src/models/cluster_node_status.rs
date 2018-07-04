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
pub struct ClusterNodeStatus {
  /// Battery status information.
  #[serde(rename = "batterystatus")]
  batterystatus: Option<::models::NodeStatusNodeBatterystatus>,
  /// Storage capacity of this node.
  #[serde(rename = "capacity")]
  capacity: Option<Vec<::models::NodeStatusNodeCapacityItem>>,
  /// CPU status information for this node.
  #[serde(rename = "cpu")]
  cpu: Option<::models::NodeStatusNodeCpu>,
  /// Node NVRAM information.
  #[serde(rename = "nvram")]
  nvram: Option<::models::NodeStatusNodeNvram>,
  /// Information about this node's power supplies.
  #[serde(rename = "powersupplies")]
  powersupplies: Option<::models::NodeStatusNodePowersupplies>,
  /// OneFS release.
  #[serde(rename = "release")]
  release: Option<String>,
  /// Seconds this node has been online.
  #[serde(rename = "uptime")]
  uptime: Option<i32>,
  /// OneFS version.
  #[serde(rename = "version")]
  version: Option<String>
}

impl ClusterNodeStatus {
  pub fn new() -> ClusterNodeStatus {
    ClusterNodeStatus {
      batterystatus: None,
      capacity: None,
      cpu: None,
      nvram: None,
      powersupplies: None,
      release: None,
      uptime: None,
      version: None
    }
  }

  pub fn set_batterystatus(&mut self, batterystatus: ::models::NodeStatusNodeBatterystatus) {
    self.batterystatus = Some(batterystatus);
  }

  pub fn with_batterystatus(mut self, batterystatus: ::models::NodeStatusNodeBatterystatus) -> ClusterNodeStatus {
    self.batterystatus = Some(batterystatus);
    self
  }

  pub fn batterystatus(&self) -> Option<&::models::NodeStatusNodeBatterystatus> {
    self.batterystatus.as_ref()
  }

  pub fn reset_batterystatus(&mut self) {
    self.batterystatus = None;
  }

  pub fn set_capacity(&mut self, capacity: Vec<::models::NodeStatusNodeCapacityItem>) {
    self.capacity = Some(capacity);
  }

  pub fn with_capacity(mut self, capacity: Vec<::models::NodeStatusNodeCapacityItem>) -> ClusterNodeStatus {
    self.capacity = Some(capacity);
    self
  }

  pub fn capacity(&self) -> Option<&Vec<::models::NodeStatusNodeCapacityItem>> {
    self.capacity.as_ref()
  }

  pub fn reset_capacity(&mut self) {
    self.capacity = None;
  }

  pub fn set_cpu(&mut self, cpu: ::models::NodeStatusNodeCpu) {
    self.cpu = Some(cpu);
  }

  pub fn with_cpu(mut self, cpu: ::models::NodeStatusNodeCpu) -> ClusterNodeStatus {
    self.cpu = Some(cpu);
    self
  }

  pub fn cpu(&self) -> Option<&::models::NodeStatusNodeCpu> {
    self.cpu.as_ref()
  }

  pub fn reset_cpu(&mut self) {
    self.cpu = None;
  }

  pub fn set_nvram(&mut self, nvram: ::models::NodeStatusNodeNvram) {
    self.nvram = Some(nvram);
  }

  pub fn with_nvram(mut self, nvram: ::models::NodeStatusNodeNvram) -> ClusterNodeStatus {
    self.nvram = Some(nvram);
    self
  }

  pub fn nvram(&self) -> Option<&::models::NodeStatusNodeNvram> {
    self.nvram.as_ref()
  }

  pub fn reset_nvram(&mut self) {
    self.nvram = None;
  }

  pub fn set_powersupplies(&mut self, powersupplies: ::models::NodeStatusNodePowersupplies) {
    self.powersupplies = Some(powersupplies);
  }

  pub fn with_powersupplies(mut self, powersupplies: ::models::NodeStatusNodePowersupplies) -> ClusterNodeStatus {
    self.powersupplies = Some(powersupplies);
    self
  }

  pub fn powersupplies(&self) -> Option<&::models::NodeStatusNodePowersupplies> {
    self.powersupplies.as_ref()
  }

  pub fn reset_powersupplies(&mut self) {
    self.powersupplies = None;
  }

  pub fn set_release(&mut self, release: String) {
    self.release = Some(release);
  }

  pub fn with_release(mut self, release: String) -> ClusterNodeStatus {
    self.release = Some(release);
    self
  }

  pub fn release(&self) -> Option<&String> {
    self.release.as_ref()
  }

  pub fn reset_release(&mut self) {
    self.release = None;
  }

  pub fn set_uptime(&mut self, uptime: i32) {
    self.uptime = Some(uptime);
  }

  pub fn with_uptime(mut self, uptime: i32) -> ClusterNodeStatus {
    self.uptime = Some(uptime);
    self
  }

  pub fn uptime(&self) -> Option<&i32> {
    self.uptime.as_ref()
  }

  pub fn reset_uptime(&mut self) {
    self.uptime = None;
  }

  pub fn set_version(&mut self, version: String) {
    self.version = Some(version);
  }

  pub fn with_version(mut self, version: String) -> ClusterNodeStatus {
    self.version = Some(version);
    self
  }

  pub fn version(&self) -> Option<&String> {
    self.version.as_ref()
  }

  pub fn reset_version(&mut self) {
    self.version = None;
  }

}


