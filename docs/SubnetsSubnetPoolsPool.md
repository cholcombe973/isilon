# SubnetsSubnetPoolsPool

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_zone** | **String** | Name of a valid access zone to map IP address pool to the zone. | [default to null]
**addr_family** | **String** | IP address format. | [default to null]
**aggregation_mode** | **String** | OneFS supports the following NIC aggregation modes. | [default to null]
**alloc_method** | **String** | Specifies how IP address allocation is done among pool members. | [default to null]
**description** | **String** | A description of the pool. | [default to null]
**groupnet** | **String** | Name of the groupnet this pool belongs to. | [default to null]
**id** | **String** | Unique Pool ID. | [default to null]
**ifaces** | [**Vec<::models::SubnetsSubnetPoolIface>**](SubnetsSubnetPoolIface.md) | List of interface members in this pool. | [default to null]
**name** | **String** | The name of the pool. It must be unique throughout the given subnet.It&#39;s a required field with POST method. | [default to null]
**ranges** | [**Vec<::models::SubnetsSubnetPoolRange>**](SubnetsSubnetPoolRange.md) | List of IP address ranges in this pool. | [default to null]
**rebalance_policy** | **String** | Rebalance policy.. | [default to null]
**rules** | **Vec<String>** | Names of the rules in this pool. | [default to null]
**sc_auto_unsuspend_delay** | **i32** | Time delay in seconds before a node which has been                 automatically unsuspended becomes usable in SmartConnect                responses for pool zones. | [default to null]
**sc_connect_policy** | **String** | SmartConnect client connection balancing policy. | [default to null]
**sc_dns_zone** | **String** | SmartConnect zone name for the pool. | [default to null]
**sc_dns_zone_aliases** | **Vec<String>** | List of SmartConnect zone aliases (DNS names) to the pool. | [default to null]
**sc_failover_policy** | **String** | SmartConnect IP failover policy. | [default to null]
**sc_subnet** | **String** | Name of SmartConnect service subnet for this pool. | [default to null]
**sc_suspended_nodes** | **Vec<i32>** | List of LNNs showing currently suspended nodes in SmartConnect. | [default to null]
**sc_ttl** | **i32** | Time to live value for SmartConnect DNS query responses in seconds. | [default to null]
**static_routes** | [**Vec<::models::SubnetsSubnetPoolStaticRoute>**](SubnetsSubnetPoolStaticRoute.md) | List of interface members in this pool. | [default to null]
**subnet** | **String** | The name of the subnet. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


