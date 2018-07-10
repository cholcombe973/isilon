#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnmpSettingsSettings {
    /// The read-only community name.  @DEFAULT reverts this field to its default value.
    #[serde(rename = "read_only_community")]
    pub read_only_community: Option<String>,
    /// Whether the SNMP service is enabled.
    #[serde(rename = "service")]
    pub service: Option<bool>,
    /// Whether SNMP v1 and v2c protocols are enabled.  @DEFAULT reverts this field to its default value.
    #[serde(rename = "snmp_v1_v2c_access")]
    pub snmp_v1_v2c_access: Option<bool>,
    /// Whether SNMP v3 is enabled.  @DEFAULT reverts this field to its default value.
    #[serde(rename = "snmp_v3_access")]
    pub snmp_v3_access: Option<bool>,
    /// SNMPv3 authentication protocol. May only be SHA or MD5.  @DEFAULT reverts this field to its default value.
    #[serde(rename = "snmp_v3_auth_protocol")]
    pub snmp_v3_auth_protocol: Option<String>,
    /// SNMPv3 privacy protocol. May only be AES or DES. @DEFAULT reverts this field to its default value.
    #[serde(rename = "snmp_v3_priv_protocol")]
    pub snmp_v3_priv_protocol: Option<String>,
    /// The read-only user for SNMP v3 read requests.  @DEFAULT reverts this field to its default value.
    #[serde(rename = "snmp_v3_read_only_user")]
    pub snmp_v3_read_only_user: Option<String>,
    /// SNMPv3 privacy protocol. May only be AES or DES. @DEFAULT reverts this field to its default value.
    #[serde(rename = "snmp_v3_security_level")]
    pub snmp_v3_security_level: Option<String>,
    /// Contact information for the system owner.  This must be a valid email address.  @DEFAULT reverts this field to its default value.
    #[serde(rename = "system_contact")]
    pub system_contact: Option<String>,
    /// A location name for the SNMP system.  @DEFAULT reverts this field to its default value.
    #[serde(rename = "system_location")]
    pub system_location: Option<String>,
}
