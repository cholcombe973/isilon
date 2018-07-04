# GroupnetSubnetExtended

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | **String** | A description of the subnet. | [optional] [default to null]
**dsr_addrs** | **Vec<String>** | List of Direct Server Return addresses. | [optional] [default to null]
**gateway** | **String** | Gateway IP address. | [optional] [default to null]
**gateway_priority** | **i32** | Gateway priority. | [optional] [default to null]
**mtu** | **i32** | MTU of the subnet. | [optional] [default to null]
**name** | **String** | The name of the subnet. | [optional] [default to null]
**prefixlen** | **i32** | Subnet Prefix Length. | [optional] [default to null]
**sc_service_addr** | **String** | The address that SmartConnect listens for DNS requests. | [optional] [default to null]
**sc_service_name** | **String** | Domain Name corresponding to the SmartConnect Service Address. | [optional] [default to null]
**vlan_enabled** | **bool** | VLAN tagging enabled or disabled. | [optional] [default to null]
**vlan_id** | **i32** | VLAN ID for all interfaces in the subnet. | [optional] [default to null]
**addr_family** | **String** | IP address format. | [optional] [default to null]
**base_addr** | **String** | The base IP address. | [optional] [default to null]
**groupnet** | **String** | Name of the groupnet this subnet belongs to. | [optional] [default to null]
**id** | **String** | Unique Subnet ID. | [optional] [default to null]
**pools** | **Vec<String>** | Name of the pools in the subnet. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


