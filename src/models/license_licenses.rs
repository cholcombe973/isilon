#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseLicenses {
    #[serde(rename = "licenses")]
    pub licenses: Option<Vec <crate::models::LicenseLicense>>,
}
