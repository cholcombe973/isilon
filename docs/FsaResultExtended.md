# FsaResultExtended

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pinned** | **bool** | True if the result is pinned to prevent automatic removal. | [default to null]
**begin_time** | **i32** | Unix Epoch time of start of results collection job. | [default to null]
**content_path** | **String** | Path to results database. | [optional] [default to null]
**delete_link** | **String** | Resource to call with DELETE to remove results.. | [optional] [default to null]
**end_time** | **i32** | Unix Epoch time of end of results collection job. | [default to null]
**fsa_state** | **String** | State of the result set. | [default to null]
**id** | **i32** | The system generated result set ID. | [default to null]
**job_state** | **Vec<String>** | State information about the FSA Job. | [default to null]
**properties_link** | **String** | Resource to call to get result properties. | [default to null]
**size** | **i32** | Size of the result set database in bytes. | [default to null]
**version** | **i32** | FSA version used to create result set. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


