/// AntivirusQuarantinePathParams : The quarantine status of a file in /ifs.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AntivirusQuarantinePathParams {
    /// If true, this file is quarantined.  If false, the file is not quarantined.
    #[serde(rename = "quarantined")]
    pub quarantined: Option<bool>,
}
