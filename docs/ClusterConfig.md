# ClusterConfig

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | **String** | Customer configurable description. | [default to null]
**devices** | [**Vec<::models::ClusterConfigDevice>**](ClusterConfigDevice.md) |  | [default to null]
**encoding** | **String** | Default encoding. | [default to null]
**guid** | **String** | Cluster GUID. | [default to null]
**has_quorum** | **bool** | If true, the local node is in a group with quorum: It is communicating, storing, and protecting data normally with other nodes in its group.  Under normal circumstances, every node in the cluster is part of one group. | [default to null]
**is_compliance** | **bool** | If true, the cluster is in compliance mode.  Compliance mode clusters do not allow root access and have stricter WORM (Write Once Read Many) features for retaining data in compliance with U.S. Securities and Exchange Commission rule 17a-4. | [default to null]
**is_virtual** | **bool** | true if the cluster is deployed on a desktop VMWorkstation | [default to null]
**is_vonefs** | **bool** | true if this is a vOneFS cluster on an ESXi | [default to null]
**join_mode** | **String** | Node join mode: &#39;manual&#39; or &#39;secure&#39;. | [default to null]
**local_devid** | **i32** | Device ID of the queried node. | [default to null]
**local_lnn** | **i32** | Device logical node number of the queried node. | [default to null]
**local_serial** | **String** | Device serial number of the queried node. | [default to null]
**name** | **String** | Cluster name. | [default to null]
**onefs_version** | [***::models::ClusterConfigOnefsVersion**](ClusterConfigOnefsVersion.md) |  | [optional] [default to null]
**timezone** | [***::models::ClusterConfigTimezone**](ClusterConfigTimezone.md) | The cluster timezone settings. | [optional] [default to null]
**upgrade_type** | **String** |  | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


