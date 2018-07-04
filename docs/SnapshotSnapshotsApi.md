# \SnapshotSnapshotsApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_snapshot_lock**](SnapshotSnapshotsApi.md#create_snapshot_lock) | **Post** /platform/1/snapshot/snapshots/{Sid}/locks | 
[**delete_snapshot_lock**](SnapshotSnapshotsApi.md#delete_snapshot_lock) | **Delete** /platform/1/snapshot/snapshots/{Sid}/locks/{SnapshotLockId} | 
[**delete_snapshot_locks**](SnapshotSnapshotsApi.md#delete_snapshot_locks) | **Delete** /platform/1/snapshot/snapshots/{Sid}/locks | 
[**get_snapshot_lock**](SnapshotSnapshotsApi.md#get_snapshot_lock) | **Get** /platform/1/snapshot/snapshots/{Sid}/locks/{SnapshotLockId} | 
[**list_snapshot_locks**](SnapshotSnapshotsApi.md#list_snapshot_locks) | **Get** /platform/1/snapshot/snapshots/{Sid}/locks | 
[**update_snapshot_lock**](SnapshotSnapshotsApi.md#update_snapshot_lock) | **Put** /platform/1/snapshot/snapshots/{Sid}/locks/{SnapshotLockId} | 


# **create_snapshot_lock**
> ::models::CreateSnapshotLockResponse create_snapshot_lock(ctx, snapshot_lock, sid)


Create a new lock on this snapshot.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **snapshot_lock** | [**SnapshotLockCreateParams**](SnapshotLockCreateParams.md)|  | 
  **sid** | **String**|  | 

### Return type

[**::models::CreateSnapshotLockResponse**](CreateSnapshotLockResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_snapshot_lock**
> delete_snapshot_lock(ctx, snapshot_lock_id, sid)


Delete the snapshot lock.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **snapshot_lock_id** | **String**| Delete the snapshot lock. | 
  **sid** | **String**|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_snapshot_locks**
> delete_snapshot_locks(ctx, sid)


Delete all locks. Will try to drain count of recursively held locks so that the snapshot can be deleted.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **sid** | **String**|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_snapshot_lock**
> ::models::SnapshotLocks get_snapshot_lock(ctx, snapshot_lock_id, sid)


Retrieve lock information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **snapshot_lock_id** | **String**| Retrieve lock information. | 
  **sid** | **String**|  | 

### Return type

[**::models::SnapshotLocks**](SnapshotLocks.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_snapshot_locks**
> ::models::SnapshotLocksExtended list_snapshot_locks(ctx, sid, optional)


List all locks.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **sid** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sid** | **String**|  | 
 **sort** | **String**| The field that will be used for sorting.  Choices are id, expires, and comment.  Default is id. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **dir** | **String**| The direction of the sort. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::SnapshotLocksExtended**](SnapshotLocksExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_snapshot_lock**
> update_snapshot_lock(ctx, snapshot_lock, snapshot_lock_id, sid)


Modify lock. All input fields are optional, but one or more must be supplied.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **snapshot_lock** | [**SnapshotLock**](SnapshotLock.md)|  | 
  **snapshot_lock_id** | **String**| Modify lock. All input fields are optional, but one or more must be supplied. | 
  **sid** | **String**|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

