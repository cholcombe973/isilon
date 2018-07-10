

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobStatisticsJobNodeMemory {
  /// 
  #[serde(rename = "physical")]
  physical: ::models::JobStatisticsJobNodeMemoryPhysical,
  /// 
  #[serde(rename = "virtual")]
  _virtual: ::models::JobStatisticsJobNodeMemoryVirtual
}

