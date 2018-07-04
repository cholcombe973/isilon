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
pub struct SnapshotSnapshots {
  #[serde(rename = "snapshots")]
  snapshots: Option<Vec<::models::SnapshotSnapshotExtended>>
}

impl SnapshotSnapshots {
  pub fn new() -> SnapshotSnapshots {
    SnapshotSnapshots {
      snapshots: None
    }
  }

  pub fn set_snapshots(&mut self, snapshots: Vec<::models::SnapshotSnapshotExtended>) {
    self.snapshots = Some(snapshots);
  }

  pub fn with_snapshots(mut self, snapshots: Vec<::models::SnapshotSnapshotExtended>) -> SnapshotSnapshots {
    self.snapshots = Some(snapshots);
    self
  }

  pub fn snapshots(&self) -> Option<&Vec<::models::SnapshotSnapshotExtended>> {
    self.snapshots.as_ref()
  }

  pub fn reset_snapshots(&mut self) {
    self.snapshots = None;
  }

}


