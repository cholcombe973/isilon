#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseLicenseCreateParams {
    /// A list of evaluation licenses to enable on the cluster.
    #[serde(rename = "evaluation")]
    pub evaluation: Option<Vec<String>>,
    /// License file string content. The license file is obtained from EMC's SLC web portal. Do not use with the license_file_path option.
    #[serde(rename = "license_file_content")]
    pub license_file_content: Option<String>,
    /// Path to new license file, must be under /ifs. The license file is obtained from EMC's SLC web portal. Do not include the path when only enabling evaluation licenses. Do not use with the license_file_content option.
    #[serde(rename = "license_file_path")]
    pub license_file_path: Option<String>,
}
