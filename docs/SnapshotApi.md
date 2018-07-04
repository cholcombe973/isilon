# \SnapshotApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_snapshot_alias**](SnapshotApi.md#create_snapshot_alias) | **Post** /platform/1/snapshot/aliases | 
[**create_snapshot_changelist**](SnapshotApi.md#create_snapshot_changelist) | **Post** /platform/1/snapshot/changelists | 
[**create_snapshot_repstate**](SnapshotApi.md#create_snapshot_repstate) | **Post** /platform/1/snapshot/repstates | 
[**create_snapshot_schedule**](SnapshotApi.md#create_snapshot_schedule) | **Post** /platform/3/snapshot/schedules | 
[**create_snapshot_snapshot**](SnapshotApi.md#create_snapshot_snapshot) | **Post** /platform/1/snapshot/snapshots | 
[**delete_snapshot_alias**](SnapshotApi.md#delete_snapshot_alias) | **Delete** /platform/1/snapshot/aliases/{SnapshotAliasId} | 
[**delete_snapshot_aliases**](SnapshotApi.md#delete_snapshot_aliases) | **Delete** /platform/1/snapshot/aliases | 
[**delete_snapshot_changelist**](SnapshotApi.md#delete_snapshot_changelist) | **Delete** /platform/1/snapshot/changelists/{SnapshotChangelistId} | 
[**delete_snapshot_repstate**](SnapshotApi.md#delete_snapshot_repstate) | **Delete** /platform/1/snapshot/repstates/{SnapshotRepstateId} | 
[**delete_snapshot_schedule**](SnapshotApi.md#delete_snapshot_schedule) | **Delete** /platform/3/snapshot/schedules/{SnapshotScheduleId} | 
[**delete_snapshot_schedules**](SnapshotApi.md#delete_snapshot_schedules) | **Delete** /platform/3/snapshot/schedules | 
[**delete_snapshot_snapshot**](SnapshotApi.md#delete_snapshot_snapshot) | **Delete** /platform/1/snapshot/snapshots/{SnapshotSnapshotId} | 
[**delete_snapshot_snapshots**](SnapshotApi.md#delete_snapshot_snapshots) | **Delete** /platform/1/snapshot/snapshots | 
[**get_snapshot_alias**](SnapshotApi.md#get_snapshot_alias) | **Get** /platform/1/snapshot/aliases/{SnapshotAliasId} | 
[**get_snapshot_changelist**](SnapshotApi.md#get_snapshot_changelist) | **Get** /platform/1/snapshot/changelists/{SnapshotChangelistId} | 
[**get_snapshot_license**](SnapshotApi.md#get_snapshot_license) | **Get** /platform/5/snapshot/license | 
[**get_snapshot_pending**](SnapshotApi.md#get_snapshot_pending) | **Get** /platform/1/snapshot/pending | 
[**get_snapshot_repstate**](SnapshotApi.md#get_snapshot_repstate) | **Get** /platform/1/snapshot/repstates/{SnapshotRepstateId} | 
[**get_snapshot_schedule**](SnapshotApi.md#get_snapshot_schedule) | **Get** /platform/3/snapshot/schedules/{SnapshotScheduleId} | 
[**get_snapshot_settings**](SnapshotApi.md#get_snapshot_settings) | **Get** /platform/1/snapshot/settings | 
[**get_snapshot_snapshot**](SnapshotApi.md#get_snapshot_snapshot) | **Get** /platform/1/snapshot/snapshots/{SnapshotSnapshotId} | 
[**get_snapshot_snapshots_summary**](SnapshotApi.md#get_snapshot_snapshots_summary) | **Get** /platform/1/snapshot/snapshots-summary | 
[**list_snapshot_aliases**](SnapshotApi.md#list_snapshot_aliases) | **Get** /platform/1/snapshot/aliases | 
[**list_snapshot_changelists**](SnapshotApi.md#list_snapshot_changelists) | **Get** /platform/1/snapshot/changelists | 
[**list_snapshot_repstates**](SnapshotApi.md#list_snapshot_repstates) | **Get** /platform/1/snapshot/repstates | 
[**list_snapshot_schedules**](SnapshotApi.md#list_snapshot_schedules) | **Get** /platform/3/snapshot/schedules | 
[**list_snapshot_snapshots**](SnapshotApi.md#list_snapshot_snapshots) | **Get** /platform/1/snapshot/snapshots | 
[**update_snapshot_alias**](SnapshotApi.md#update_snapshot_alias) | **Put** /platform/1/snapshot/aliases/{SnapshotAliasId} | 
[**update_snapshot_schedule**](SnapshotApi.md#update_snapshot_schedule) | **Put** /platform/3/snapshot/schedules/{SnapshotScheduleId} | 
[**update_snapshot_settings**](SnapshotApi.md#update_snapshot_settings) | **Put** /platform/1/snapshot/settings | 
[**update_snapshot_snapshot**](SnapshotApi.md#update_snapshot_snapshot) | **Put** /platform/1/snapshot/snapshots/{SnapshotSnapshotId} | 


# **create_snapshot_alias**
> ::models::CreateSnapshotAliasResponse create_snapshot_alias(ctx, snapshot_alias)


Create a new snapshot alias.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **snapshot_alias** | [**SnapshotAliasCreateParams**](SnapshotAliasCreateParams.md)|  | 

### Return type

[**::models::CreateSnapshotAliasResponse**](CreateSnapshotAliasResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_snapshot_changelist**
> ::models::CreateSnapshotChangelistResponse create_snapshot_changelist(ctx, snapshot_changelist)


Create a new changelist.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **snapshot_changelist** | [**SnapshotChangelists**](SnapshotChangelists.md)|  | 

### Return type

[**::models::CreateSnapshotChangelistResponse**](CreateSnapshotChangelistResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_snapshot_repstate**
> ::models::CreateSnapshotRepstateResponse create_snapshot_repstate(ctx, snapshot_repstate)


Create a new repstates.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **snapshot_repstate** | [**SnapshotRepstates**](SnapshotRepstates.md)|  | 

### Return type

[**::models::CreateSnapshotRepstateResponse**](CreateSnapshotRepstateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_snapshot_schedule**
> ::models::CreateSnapshotScheduleResponse create_snapshot_schedule(ctx, snapshot_schedule)


Create a new schedule.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **snapshot_schedule** | [**SnapshotScheduleCreateParams**](SnapshotScheduleCreateParams.md)|  | 

### Return type

[**::models::CreateSnapshotScheduleResponse**](CreateSnapshotScheduleResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_snapshot_snapshot**
> ::models::SnapshotSnapshotExtended create_snapshot_snapshot(ctx, snapshot_snapshot)


Create a new snapshot.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **snapshot_snapshot** | [**SnapshotSnapshotCreateParams**](SnapshotSnapshotCreateParams.md)|  | 

### Return type

[**::models::SnapshotSnapshotExtended**](SnapshotSnapshotExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_snapshot_alias**
> delete_snapshot_alias(ctx, snapshot_alias_id)


Delete the snapshot alias

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **snapshot_alias_id** | **String**| Delete the snapshot alias | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_snapshot_aliases**
> delete_snapshot_aliases(ctx, )


Delete all or matching snapshot aliases.

### Required Parameters
This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_snapshot_changelist**
> delete_snapshot_changelist(ctx, snapshot_changelist_id)


Delete the specified changelist.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **snapshot_changelist_id** | **String**| Delete the specified changelist. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_snapshot_repstate**
> delete_snapshot_repstate(ctx, snapshot_repstate_id)


Delete the specified repstate.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **snapshot_repstate_id** | **String**| Delete the specified repstate. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_snapshot_schedule**
> delete_snapshot_schedule(ctx, snapshot_schedule_id)


Delete the schedule. This does not affect already created snapshots.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **snapshot_schedule_id** | **String**| Delete the schedule. This does not affect already created snapshots. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_snapshot_schedules**
> delete_snapshot_schedules(ctx, )


Delete all snapshot schedules.

### Required Parameters
This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_snapshot_snapshot**
> delete_snapshot_snapshot(ctx, snapshot_snapshot_id)


Delete the snapshot. Deleted snapshots will be placed into a deleting state until the system can reclaim the space used by the snapshot.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **snapshot_snapshot_id** | **String**| Delete the snapshot. Deleted snapshots will be placed into a deleting state until the system can reclaim the space used by the snapshot. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_snapshot_snapshots**
> delete_snapshot_snapshots(ctx, optional)


Delete all or matching snapshots.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **_type** | **String**| Only list snapshots matching this type. | 
 **schedule** | **String**| Only list snapshots created by this schedule. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_snapshot_alias**
> ::models::SnapshotAliases get_snapshot_alias(ctx, snapshot_alias_id)


Retrieve snapshot alias information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **snapshot_alias_id** | **String**| Retrieve snapshot alias information. | 

### Return type

[**::models::SnapshotAliases**](SnapshotAliases.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_snapshot_changelist**
> ::models::SnapshotChangelists get_snapshot_changelist(ctx, snapshot_changelist_id, optional)


Retrieve basic information on a changelist.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **snapshot_changelist_id** | **String**| Retrieve basic information on a changelist. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **snapshot_changelist_id** | **String**| Retrieve basic information on a changelist. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::SnapshotChangelists**](SnapshotChangelists.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_snapshot_license**
> ::models::LicenseLicense get_snapshot_license(ctx, )


Retrieve license information.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::LicenseLicense**](LicenseLicense.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_snapshot_pending**
> ::models::SnapshotPending get_snapshot_pending(ctx, optional)


Return list of snapshots to be taken.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **limit** | **i32**| Return no more than this many result at once (see resume). | 
 **begin** | **i32**| Unix Epoch time to start generating matches. Default is now. | 
 **schedule** | **String**| Limit output only to the named schedule. | 
 **end** | **i32**| Unix Epoch time to end generating matches. Default is forever. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::SnapshotPending**](SnapshotPending.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_snapshot_repstate**
> ::models::SnapshotRepstates get_snapshot_repstate(ctx, snapshot_repstate_id, optional)


Retrieve basic information on a repstate.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **snapshot_repstate_id** | **String**| Retrieve basic information on a repstate. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **snapshot_repstate_id** | **String**| Retrieve basic information on a repstate. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::SnapshotRepstates**](SnapshotRepstates.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_snapshot_schedule**
> ::models::SnapshotSchedules get_snapshot_schedule(ctx, snapshot_schedule_id)


Retrieve the schedule.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **snapshot_schedule_id** | **String**| Retrieve the schedule. | 

### Return type

[**::models::SnapshotSchedules**](SnapshotSchedules.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_snapshot_settings**
> ::models::SnapshotSettings get_snapshot_settings(ctx, )


List all settings

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::SnapshotSettings**](SnapshotSettings.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_snapshot_snapshot**
> ::models::SnapshotSnapshots get_snapshot_snapshot(ctx, snapshot_snapshot_id)


Retrieve snapshot information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **snapshot_snapshot_id** | **String**| Retrieve snapshot information. | 

### Return type

[**::models::SnapshotSnapshots**](SnapshotSnapshots.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_snapshot_snapshots_summary**
> ::models::SnapshotSnapshotsSummary get_snapshot_snapshots_summary(ctx, )


Return summary information about snapshots.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::SnapshotSnapshotsSummary**](SnapshotSnapshotsSummary.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_snapshot_aliases**
> ::models::SnapshotAliasesExtended list_snapshot_aliases(ctx, optional)


List all or matching snapshot aliases.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sort** | **String**| The field that will be used for sorting.  Choices are id, name, snapshot, and created.  Default is id. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **dir** | **String**| The direction of the sort. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::SnapshotAliasesExtended**](SnapshotAliasesExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_snapshot_changelists**
> ::models::SnapshotChangelistsExtended list_snapshot_changelists(ctx, optional)


List all changelists.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::SnapshotChangelistsExtended**](SnapshotChangelistsExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_snapshot_repstates**
> ::models::SnapshotRepstatesExtended list_snapshot_repstates(ctx, optional)


List all repstates.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::SnapshotRepstatesExtended**](SnapshotRepstatesExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_snapshot_schedules**
> ::models::SnapshotSchedulesExtended list_snapshot_schedules(ctx, optional)


List all or matching schedules.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sort** | **String**| The field that will be used for sorting.  Choices are id, name, path, pattern, schedule, duration, alias, next_run, and next_snapshot.  Default is id. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **dir** | **String**| The direction of the sort. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::SnapshotSchedulesExtended**](SnapshotSchedulesExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_snapshot_snapshots**
> ::models::SnapshotSnapshotsExtended list_snapshot_snapshots(ctx, optional)


List all or matching snapshots.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sort** | **String**| The field that will be used for sorting.  Choices are id, name, path, created, expires, size, has_locks, schedule, alias_target, alias_target_name, pct_filesystem, pct_reserve, and state.  Default is id. | 
 **schedule** | **String**| Only list snapshots created by this schedule. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 
 **state** | **String**| Only list snapshots matching this state. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **_type** | **String**| Only list snapshots matching this type. | 
 **dir** | **String**| The direction of the sort. | 

### Return type

[**::models::SnapshotSnapshotsExtended**](SnapshotSnapshotsExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_snapshot_alias**
> update_snapshot_alias(ctx, snapshot_alias, snapshot_alias_id)


Modify snapshot alias. All input fields are optional, but one or more must be supplied.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **snapshot_alias** | [**SnapshotAlias**](SnapshotAlias.md)|  | 
  **snapshot_alias_id** | **String**| Modify snapshot alias. All input fields are optional, but one or more must be supplied. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_snapshot_schedule**
> update_snapshot_schedule(ctx, snapshot_schedule, snapshot_schedule_id)


Modify the schedule. All input fields are optional, but one or more must be supplied.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **snapshot_schedule** | [**SnapshotSchedule**](SnapshotSchedule.md)|  | 
  **snapshot_schedule_id** | **String**| Modify the schedule. All input fields are optional, but one or more must be supplied. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_snapshot_settings**
> update_snapshot_settings(ctx, snapshot_settings)


Modify one or more settings.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **snapshot_settings** | [**SnapshotSettingsExtended**](SnapshotSettingsExtended.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_snapshot_snapshot**
> update_snapshot_snapshot(ctx, snapshot_snapshot, snapshot_snapshot_id)


Modify snapshot. All input fields are optional, but one or more must be supplied.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **snapshot_snapshot** | [**SnapshotSnapshot**](SnapshotSnapshot.md)|  | 
  **snapshot_snapshot_id** | **String**| Modify snapshot. All input fields are optional, but one or more must be supplied. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

