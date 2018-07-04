# NfsNlmLocksLock

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client** | **String** | Specifies the client host name and IP address. | [optional] [default to null]
**client_id** | **String** | Specifies the client ID. | [optional] [default to null]
**created** | **i32** | Specifies the UNIX EPoch time that the lock was created. | [optional] [default to null]
**id** | **String** | Specifies the system-assigned ID given to the lock. This value is returned when the lock is created with the POST method. | [optional] [default to null]
**lin** | **String** | Specifies the LIN in /ifs that is locked. | [optional] [default to null]
**lock_type** | **String** | Specifies the lock type. | [optional] [default to null]
**path** | **String** | Specifies the path under /ifs that is locked. | [optional] [default to null]
**range** | **Vec<i32>** | Specifies the byte range within the locked file. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


