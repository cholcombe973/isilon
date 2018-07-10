#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobStatisticsJobNodeMemory {
    ///
    #[serde(rename = "physical")]
    pub physical: ::models::JobStatisticsJobNodeMemoryPhysical,
    ///
    #[serde(rename = "virtual")]
    pub _virtual: ::models::JobStatisticsJobNodeMemoryVirtual,
}
