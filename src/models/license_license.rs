#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseLicense {
    /// Number of days since a license expired.
    #[serde(rename = "days_since_expiry")]
    pub days_since_expiry: Option<i32>,
    /// Number of days before a license expires.
    #[serde(rename = "days_to_expiry")]
    pub days_to_expiry: Option<i32>,
    /// Date of license expiry. Format is YYYY-MM-DD. It is not included if there is no expiration. Feature is considered expired at end of this day. The cluster time is used to determine expiry.
    #[serde(rename = "expiration")]
    pub expiration: Option<String>,
    /// True when we are generating an alert that this feature has expired.
    #[serde(rename = "expired_alert")]
    pub expired_alert: bool,
    /// True when we are generating an alert that this feature is expiring.
    #[serde(rename = "expiring_alert")]
    pub expiring_alert: bool,
    /// Name of the licensed feature.
    #[serde(rename = "id")]
    pub id: String,
    /// Name of the licensed feature.
    #[serde(rename = "name")]
    pub name: String,
    /// Current status of the license.
    #[serde(rename = "status")]
    pub status: String,
    /// Tiered License details.
    #[serde(rename = "tiers")]
    pub tiers: Vec <crate::models::LicenseLicenseTier>,
}
