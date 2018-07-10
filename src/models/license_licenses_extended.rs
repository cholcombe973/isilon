#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseLicensesExtended {
    #[serde(rename = "licenses")]
    pub licenses: Option<Vec<::models::LicenseLicense>>,
    /// True when we are generating an activation incomplete alert. An activation incomplete alert is generated if we do not have a signed license file 90 days after OneFS is upgraded.
    #[serde(rename = "activation_incomplete_alert")]
    pub activation_incomplete_alert: bool,
    #[serde(rename = "base_only_licenses")]
    pub base_only_licenses: Vec<String>,
    #[serde(rename = "evaluatable")]
    pub evaluatable: Vec<String>,
    /// Software license identifier. SWID will be absent if not yet obtained from a license file.
    #[serde(rename = "swid")]
    pub swid: Option<String>,
    /// True if license file contains a valid signature.
    #[serde(rename = "valid_signature")]
    pub valid_signature: bool,
}
