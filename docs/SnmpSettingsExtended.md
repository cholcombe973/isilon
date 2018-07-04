# SnmpSettingsExtended

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**read_only_community** | **String** | The read-only community name.  @DEFAULT reverts this field to its default value. | [optional] [default to null]
**service** | **bool** | Whether the SNMP service is enabled. | [optional] [default to null]
**snmp_v1_v2c_access** | **bool** | Whether SNMP v1 and v2c protocols are enabled.  @DEFAULT reverts this field to its default value. | [optional] [default to null]
**snmp_v3_access** | **bool** | Whether SNMP v3 is enabled.  @DEFAULT reverts this field to its default value. | [optional] [default to null]
**snmp_v3_auth_protocol** | **String** | SNMPv3 authentication protocol. May only be SHA or MD5.  @DEFAULT reverts this field to its default value. | [optional] [default to null]
**snmp_v3_password** | **String** | This field allows a client to change the SNMP v3 authentication password. There is always a password set.  @DEFAULT reverts this field to its default value. | [optional] [default to null]
**snmp_v3_priv_password** | **String** | This field allows a client to change the SNMP v3 privacy password. There is always a password set.  @DEFAULT reverts this field to its default value. | [optional] [default to null]
**snmp_v3_priv_protocol** | **String** | SNMPv3 privacy protocol. May only be AES or DES. @DEFAULT reverts this field to its default value. | [optional] [default to null]
**snmp_v3_read_only_user** | **String** | The read-only user for SNMP v3 read requests.  @DEFAULT reverts this field to its default value. | [optional] [default to null]
**snmp_v3_security_level** | **String** | SNMPv3 privacy protocol. May only be AES or DES. @DEFAULT reverts this field to its default value. | [optional] [default to null]
**system_contact** | **String** | Contact information for the system owner.  This must be a valid email address.  @DEFAULT reverts this field to its default value. | [optional] [default to null]
**system_location** | **String** | A location name for the SNMP system.  @DEFAULT reverts this field to its default value. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


