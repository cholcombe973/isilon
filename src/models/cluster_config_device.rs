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
pub struct ClusterConfigDevice {
  /// Device ID.
  #[serde(rename = "devid")]
  devid: i32,
  /// Device GUID.
  #[serde(rename = "guid")]
  guid: String,
  /// If true, this node is online and communicating with the local node and every other node with the is_up property normally.  If false, this node is not currently communicating with the local node or other nodes with the is_up property.  It may be shut down, rebooting, disconnected from the backend network, or connected only to other nodes.
  #[serde(rename = "is_up")]
  is_up: bool,
  /// Device logical node number.
  #[serde(rename = "lnn")]
  lnn: i32
}

impl ClusterConfigDevice {
  pub fn new(devid: i32, guid: String, is_up: bool, lnn: i32) -> ClusterConfigDevice {
    ClusterConfigDevice {
      devid: devid,
      guid: guid,
      is_up: is_up,
      lnn: lnn
    }
  }

  pub fn set_devid(&mut self, devid: i32) {
    self.devid = devid;
  }

  pub fn with_devid(mut self, devid: i32) -> ClusterConfigDevice {
    self.devid = devid;
    self
  }

  pub fn devid(&self) -> &i32 {
    &self.devid
  }


  pub fn set_guid(&mut self, guid: String) {
    self.guid = guid;
  }

  pub fn with_guid(mut self, guid: String) -> ClusterConfigDevice {
    self.guid = guid;
    self
  }

  pub fn guid(&self) -> &String {
    &self.guid
  }


  pub fn set_is_up(&mut self, is_up: bool) {
    self.is_up = is_up;
  }

  pub fn with_is_up(mut self, is_up: bool) -> ClusterConfigDevice {
    self.is_up = is_up;
    self
  }

  pub fn is_up(&self) -> &bool {
    &self.is_up
  }


  pub fn set_lnn(&mut self, lnn: i32) {
    self.lnn = lnn;
  }

  pub fn with_lnn(mut self, lnn: i32) -> ClusterConfigDevice {
    self.lnn = lnn;
    self
  }

  pub fn lnn(&self) -> &i32 {
    &self.lnn
  }


}


