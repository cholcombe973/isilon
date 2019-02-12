# PoolsPoolInterfacesInterface

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique interface ID. | [default to null]
**ip_addrs** | **Vec<String>** | List of IP addresses | [default to null]
**lnn** | **i32** | Logical Node Number | [default to null]
**name** | **String** | The name of the interface. | [default to null]
**nic_name** | **String** | NIC name | [default to null]
**owners** | [**Vec <crate::models::PoolsPoolInterfacesInterfaceOwner>**](PoolsPoolInterfacesInterfaceOwner.md) | List of owners (membership) | [default to null]
**status** | **String** | Status of the interface | [default to null]
**_type** | **String** | Interface type.  The &#39;*gige&#39; types stand for &#39;gigabit ethernet&#39;.  &#39;gige&#39; itself is occasionally also referred to in other places as &#39;ext&#39; for &#39;external&#39;.  &#39;ib&#39; and &#39;ib_qdr&#39; are internal Infiniband interface types.  &#39;vlan&#39; and &#39;vmxnet3&#39; are virtual interface types that appear on virtual nodes.  &#39;loopback&#39; is an interface for failover addresses and should only appear if failover is configured. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


