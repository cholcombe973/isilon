#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseGenerateHardwareItem {
    /// Licensed terabyte (TB, 10^12 bytes) drive capacity allocated as storage associated with tier. Included if tier is not NONINF and license is not a base only license.
    #[serde(rename = "drive_capacity")]
    pub drive_capacity: Option<i32>,
    /// Licensed number of nodes in this tier.
    #[serde(rename = "node_count")]
    pub node_count: Option<i32>,
    /// Licensed number of nodes of this tier that contain self-encrypting drives. Included only if license is ONEFS and tier is not NONINF.
    #[serde(rename = "nodes_with_seds_count")]
    pub nodes_with_seds_count: Option<i32>,
    /// OneFS hardware tier. Tier is a number, NONINF, or NO_TIER. NONINF indicates a non infinity tier. NO_TIER indicates a license that is not tier based.
    #[serde(rename = "tier")]
    pub tier: Option<String>,
}
