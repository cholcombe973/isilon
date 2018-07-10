/// HardeningApplyItem : Apply hardening on the cluster.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HardeningApplyItem {
    /// Hardening profile.
    #[serde(rename = "profile")]
    pub profile: Option<String>,
    /// Option to only generate and display a report on current cluster configuration with respect to the expected configuation required to apply hardening. If his option is set to true, hardening is not applied after the report is displayed. By default, this option is false.
    #[serde(rename = "report")]
    pub report: Option<bool>,
}
