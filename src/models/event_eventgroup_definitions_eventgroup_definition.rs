#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventEventgroupDefinitionsEventgroupDefinition {
    /// ID of eventgroup category: all, 100000000 (SYS_DISK_EVENTS), 200000000 (NODE_STATUS_EVENTS), 300000000 (REBOOT_EVENTS), 400000000 (SW_EVENTS), 500000000 (QUOTA_EVENTS), 600000000 (SNAP_EVENTS), 700000000 (WINNET_EVENTS), 800000000 (FILESYS_EVENTS), 900000000 (HW_EVENTS), 1100000000 (CPOOL_EVENTS)
    #[serde(rename = "category")]
    pub category: Option<String>,
    /// Human readable description - may contain value placeholders.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Unique Identifier.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Name for eventgroup.
    #[serde(rename = "name")]
    pub name: Option<String>,
}
