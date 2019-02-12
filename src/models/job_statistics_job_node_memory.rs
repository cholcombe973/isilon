#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobStatisticsJobNodeMemory {
    ///
    #[serde(rename = "physical")]
    pub physical:crate::models::JobStatisticsJobNodeMemoryPhysical,
    ///
    #[serde(rename = "virtual")]
    pub _virtual:crate::models::JobStatisticsJobNodeMemoryVirtual,
}
