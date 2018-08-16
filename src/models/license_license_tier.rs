#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseLicenseTier {
    /// List of alerts about exceeded entitlements: The following alerts appear when usage of a resource such as a node, an encryption node, or storage capacity exceeds the quantity licensed for that resource.
    #[serde(rename = "entitlements_exceeded_alerts")]
    pub entitlements_exceeded_alerts:
        Option<Vec<::models::LicenseLicenseTierEntitlementsExceededAlert>>,
    /// Licensed terabyte (TB, 10^12 bytes) drive capacity allocated as storage associated with tier. Included if tier is not NONINF and license is not a base only license.
    #[serde(rename = "licensed_drive_capacity")]
    pub licensed_drive_capacity: Option<u64>,
    /// Licensed number of nodes in this tier.
    #[serde(rename = "licensed_node_count")]
    pub licensed_node_count: Option<i32>,
    /// Licensed number of nodes of this tier that contain self-encrypting drives. Included only if license is ONEFS and tier is not NONINF.
    #[serde(rename = "licensed_nodes_with_seds_count")]
    pub licensed_nodes_with_seds_count: Option<i32>,
    /// OneFS hardware tier. Tier is a number, NONINF, or NO_TIER. NONINF indicates a non infinity tier. NO_TIER indicates a license that is not tier based.
    #[serde(rename = "tier")]
    pub tier: Option<String>,
    /// Actual terabyte (TB, 10^12 bytes) drive capacity allocated as storage space associated with tier. Included if tier is not NONINF and license is not a base only license.
    #[serde(rename = "used_drive_capacity")]
    pub used_drive_capacity: Option<u64>,
    /// Actual number of nodes in this tier.
    #[serde(rename = "used_node_count")]
    pub used_node_count: Option<i32>,
    /// Actual number of nodes of this tier that contain self-encrypting drives. Included only if license is ONEFS and if tier is not NONINF.
    #[serde(rename = "used_nodes_with_seds_count")]
    pub used_nodes_with_seds_count: Option<i32>,
}
