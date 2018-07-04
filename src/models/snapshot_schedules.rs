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
pub struct SnapshotSchedules {
  #[serde(rename = "schedules")]
  schedules: Option<Vec<::models::SnapshotScheduleExtended>>
}

impl SnapshotSchedules {
  pub fn new() -> SnapshotSchedules {
    SnapshotSchedules {
      schedules: None
    }
  }

  pub fn set_schedules(&mut self, schedules: Vec<::models::SnapshotScheduleExtended>) {
    self.schedules = Some(schedules);
  }

  pub fn with_schedules(mut self, schedules: Vec<::models::SnapshotScheduleExtended>) -> SnapshotSchedules {
    self.schedules = Some(schedules);
    self
  }

  pub fn schedules(&self) -> Option<&Vec<::models::SnapshotScheduleExtended>> {
    self.schedules.as_ref()
  }

  pub fn reset_schedules(&mut self) {
    self.schedules = None;
  }

}


