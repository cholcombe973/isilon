# HdfsRackExtended

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_ip_ranges** | **Vec<String>** | Array of IP ranges. Clients from one of these IP ranges are served by corresponding nodes from ip_pools array. | [optional] [default to null]
**ip_pools** | **Vec<String>** | Array of IP pool names to use for serving clients from client_ip_ranges. | [optional] [default to null]
**name** | **String** | Name of the rack | [optional] [default to null]
**id** | **String** | The ID of the rack. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


