# NfsNlmSessionsSession

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**delegates** | **Vec<i32>** |  | [optional] [default to null]
**host_type** | **String** | The sort of host that this entry represents | [optional] [default to null]
**hostname** | **String** | The host being monitored | [optional] [default to null]
**is_active** | **bool** | Whether or not the client is actively being monitored | [optional] [default to null]
**last_modified** | **i32** | Unix time in seconds that the client was last modified (monitored or unmonitored) | [optional] [default to null]
**node_ip** | **String** | An IP address for which NSM has client records | [optional] [default to null]
**notify_attempts_remaining** | **i32** | Number of times we will attempt to notify this client before giving up | [optional] [default to null]
**notify_error** | **String** | Last error recieved attempting to notify this client | [optional] [default to null]
**notify_last_attempt** | **i32** | Unix time in seconds when we last attempted to notify this clients | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


