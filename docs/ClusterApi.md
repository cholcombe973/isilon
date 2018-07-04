# \ClusterApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_cluster_add_node_item**](ClusterApi.md#create_cluster_add_node_item) | **Post** /platform/3/cluster/add-node | 
[**create_diagnostics_gather_start_item**](ClusterApi.md#create_diagnostics_gather_start_item) | **Post** /platform/3/cluster/diagnostics/gather/start | 
[**create_diagnostics_gather_stop_item**](ClusterApi.md#create_diagnostics_gather_stop_item) | **Post** /platform/3/cluster/diagnostics/gather/stop | 
[**create_diagnostics_netlogger_start_item**](ClusterApi.md#create_diagnostics_netlogger_start_item) | **Post** /platform/3/cluster/diagnostics/netlogger/start | 
[**create_diagnostics_netlogger_stop_item**](ClusterApi.md#create_diagnostics_netlogger_stop_item) | **Post** /platform/3/cluster/diagnostics/netlogger/stop | 
[**get_cluster_config**](ClusterApi.md#get_cluster_config) | **Get** /platform/3/cluster/config | 
[**get_cluster_email**](ClusterApi.md#get_cluster_email) | **Get** /platform/1/cluster/email | 
[**get_cluster_external_ips**](ClusterApi.md#get_cluster_external_ips) | **Get** /platform/2/cluster/external-ips | 
[**get_cluster_identity**](ClusterApi.md#get_cluster_identity) | **Get** /platform/5/cluster/identity | 
[**get_cluster_node**](ClusterApi.md#get_cluster_node) | **Get** /platform/5/cluster/nodes/{ClusterNodeId} | 
[**get_cluster_nodes**](ClusterApi.md#get_cluster_nodes) | **Get** /platform/5/cluster/nodes | 
[**get_cluster_nodes_available**](ClusterApi.md#get_cluster_nodes_available) | **Get** /platform/3/cluster/nodes-available | 
[**get_cluster_owner**](ClusterApi.md#get_cluster_owner) | **Get** /platform/1/cluster/owner | 
[**get_cluster_statfs**](ClusterApi.md#get_cluster_statfs) | **Get** /platform/1/cluster/statfs | 
[**get_cluster_time**](ClusterApi.md#get_cluster_time) | **Get** /platform/3/cluster/time | 
[**get_cluster_timezone**](ClusterApi.md#get_cluster_timezone) | **Get** /platform/3/cluster/timezone | 
[**get_cluster_version**](ClusterApi.md#get_cluster_version) | **Get** /platform/3/cluster/version | 
[**get_diagnostics_gather**](ClusterApi.md#get_diagnostics_gather) | **Get** /platform/3/cluster/diagnostics/gather | 
[**get_diagnostics_gather_settings**](ClusterApi.md#get_diagnostics_gather_settings) | **Get** /platform/3/cluster/diagnostics/gather/settings | 
[**get_diagnostics_gather_status**](ClusterApi.md#get_diagnostics_gather_status) | **Get** /platform/3/cluster/diagnostics/gather/status | 
[**get_diagnostics_netlogger**](ClusterApi.md#get_diagnostics_netlogger) | **Get** /platform/3/cluster/diagnostics/netlogger | 
[**get_diagnostics_netlogger_settings**](ClusterApi.md#get_diagnostics_netlogger_settings) | **Get** /platform/3/cluster/diagnostics/netlogger/settings | 
[**get_diagnostics_netlogger_status**](ClusterApi.md#get_diagnostics_netlogger_status) | **Get** /platform/3/cluster/diagnostics/netlogger/status | 
[**get_timezone_region**](ClusterApi.md#get_timezone_region) | **Get** /platform/3/cluster/timezone/regions/{TimezoneRegionId} | 
[**get_timezone_settings**](ClusterApi.md#get_timezone_settings) | **Get** /platform/3/cluster/timezone/settings | 
[**update_cluster_email**](ClusterApi.md#update_cluster_email) | **Put** /platform/1/cluster/email | 
[**update_cluster_node**](ClusterApi.md#update_cluster_node) | **Put** /platform/5/cluster/nodes/{ClusterNodeId} | 
[**update_cluster_owner**](ClusterApi.md#update_cluster_owner) | **Put** /platform/1/cluster/owner | 
[**update_cluster_time**](ClusterApi.md#update_cluster_time) | **Put** /platform/3/cluster/time | 
[**update_cluster_timezone**](ClusterApi.md#update_cluster_timezone) | **Put** /platform/3/cluster/timezone | 
[**update_diagnostics_gather_settings**](ClusterApi.md#update_diagnostics_gather_settings) | **Put** /platform/3/cluster/diagnostics/gather/settings | 
[**update_diagnostics_netlogger_settings**](ClusterApi.md#update_diagnostics_netlogger_settings) | **Put** /platform/3/cluster/diagnostics/netlogger/settings | 
[**update_timezone_settings**](ClusterApi.md#update_timezone_settings) | **Put** /platform/3/cluster/timezone/settings | 


# **create_cluster_add_node_item**
> ::models::Empty create_cluster_add_node_item(ctx, cluster_add_node_item)


Serial number and arguments of node to add.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cluster_add_node_item** | [**ClusterAddNodeItem**](ClusterAddNodeItem.md)|  | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_diagnostics_gather_start_item**
> ::models::Empty create_diagnostics_gather_start_item(ctx, diagnostics_gather_start_item)


Start a new gather

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **diagnostics_gather_start_item** | [**DiagnosticsGatherSettingsExtended**](DiagnosticsGatherSettingsExtended.md)|  | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_diagnostics_gather_stop_item**
> ::models::Empty create_diagnostics_gather_stop_item(ctx, diagnostics_gather_stop_item)


Stop a running gather

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **diagnostics_gather_stop_item** | [**Empty**](Empty.md)|  | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_diagnostics_netlogger_start_item**
> ::models::Empty create_diagnostics_netlogger_start_item(ctx, diagnostics_netlogger_start_item)


Start a new packet caputre

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **diagnostics_netlogger_start_item** | [**DiagnosticsNetloggerSettings**](DiagnosticsNetloggerSettings.md)|  | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_diagnostics_netlogger_stop_item**
> ::models::Empty create_diagnostics_netlogger_stop_item(ctx, diagnostics_netlogger_stop_item)


Stop a running packet capture

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **diagnostics_netlogger_stop_item** | [**Empty**](Empty.md)|  | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_cluster_config**
> ::models::ClusterConfig get_cluster_config(ctx, )


Retrieve the cluster information.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::ClusterConfig**](ClusterConfig.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_cluster_email**
> ::models::ClusterEmail get_cluster_email(ctx, )


Get the cluster email notification settings.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::ClusterEmail**](ClusterEmail.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_cluster_external_ips**
> Vec<String> get_cluster_external_ips(ctx, )


Retrieve the cluster IP addresses including IPV4 and IPV6.

### Required Parameters
This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_cluster_identity**
> ::models::ClusterIdentity get_cluster_identity(ctx, )


Retrieve the login information.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::ClusterIdentity**](ClusterIdentity.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_cluster_node**
> ::models::ClusterNodesExtendedExtended get_cluster_node(ctx, cluster_node_id, optional)


Retrieve node information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cluster_node_id** | **i32**| Retrieve node information. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **cluster_node_id** | **i32**| Retrieve node information. | 
 **timeout** | **f32**| Request timeout | 

### Return type

[**::models::ClusterNodesExtendedExtended**](ClusterNodesExtendedExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_cluster_nodes**
> ::models::ClusterNodesExtendedExtended get_cluster_nodes(ctx, optional)


List the nodes on this cluster.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **timeout** | **f32**| Request timeout | 

### Return type

[**::models::ClusterNodesExtendedExtended**](ClusterNodesExtendedExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_cluster_nodes_available**
> ::models::ClusterNodesAvailable get_cluster_nodes_available(ctx, )


List all nodes that are available to add to this cluster.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::ClusterNodesAvailable**](ClusterNodesAvailable.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_cluster_owner**
> ::models::ClusterOwner get_cluster_owner(ctx, )


Get the cluster contact info settings

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::ClusterOwner**](ClusterOwner.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_cluster_statfs**
> ::models::ClusterStatfs get_cluster_statfs(ctx, )


Retrieve the filesystem statistics.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::ClusterStatfs**](ClusterStatfs.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_cluster_time**
> ::models::ClusterTime get_cluster_time(ctx, )


Retrieve the current time as reported by each node.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::ClusterTime**](ClusterTime.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_cluster_timezone**
> ::models::ClusterTimezone get_cluster_timezone(ctx, )


Get the cluster timezone.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::ClusterTimezone**](ClusterTimezone.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_cluster_version**
> ::models::ClusterVersion get_cluster_version(ctx, )


Retrieve the OneFS version as reported by each node.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::ClusterVersion**](ClusterVersion.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_diagnostics_gather**
> ::models::DiagnosticsGatherStatus get_diagnostics_gather(ctx, )


Get the status of isi_gather_info.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::DiagnosticsGatherStatus**](DiagnosticsGatherStatus.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_diagnostics_gather_settings**
> ::models::DiagnosticsGatherSettings get_diagnostics_gather_settings(ctx, )


Get the default options for isi_gather_info.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::DiagnosticsGatherSettings**](DiagnosticsGatherSettings.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_diagnostics_gather_status**
> ::models::DiagnosticsGatherStatus get_diagnostics_gather_status(ctx, )


Get the status of isi_gather_info.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::DiagnosticsGatherStatus**](DiagnosticsGatherStatus.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_diagnostics_netlogger**
> ::models::DiagnosticsNetloggerStatus get_diagnostics_netlogger(ctx, )


Get the status of isi_netlogger.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::DiagnosticsNetloggerStatus**](DiagnosticsNetloggerStatus.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_diagnostics_netlogger_settings**
> ::models::DiagnosticsNetloggerSettings get_diagnostics_netlogger_settings(ctx, )


Get the default options for isi_netlogger.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::DiagnosticsNetloggerSettings**](DiagnosticsNetloggerSettings.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_diagnostics_netlogger_status**
> ::models::DiagnosticsNetloggerStatus get_diagnostics_netlogger_status(ctx, )


Get the status of isi_netlogger.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::DiagnosticsNetloggerStatus**](DiagnosticsNetloggerStatus.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_timezone_region**
> ::models::TimezoneRegions get_timezone_region(ctx, timezone_region_id, optional)


List timezone regions.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **timezone_region_id** | **String**| List timezone regions. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **timezone_region_id** | **String**| List timezone regions. | 
 **sort** | **String**| The field that will be used for sorting. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 
 **show_all** | **bool**| Show all timezones within the region specified in the URI. | 
 **dst_reset** | **bool**| This query arg is not needed in normal use cases. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **dir** | **String**| The direction of the sort. | 

### Return type

[**::models::TimezoneRegions**](TimezoneRegions.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_timezone_settings**
> ::models::TimezoneSettings get_timezone_settings(ctx, )


Retrieve the cluster timezone.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::TimezoneSettings**](TimezoneSettings.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_cluster_email**
> update_cluster_email(ctx, cluster_email)


Modify the cluster email notification settings.  All input fields are optional, but one or more must be supplied.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cluster_email** | [**ClusterEmailExtended**](ClusterEmailExtended.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_cluster_node**
> update_cluster_node(ctx, cluster_node, cluster_node_id)


Modify one or more node settings.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cluster_node** | [**ClusterNode**](ClusterNode.md)|  | 
  **cluster_node_id** | **i32**| Modify one or more node settings. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_cluster_owner**
> update_cluster_owner(ctx, cluster_owner)


Modify the cluster contact info settings.  All input fields are optional, but one or more must be supplied.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cluster_owner** | [**ClusterOwner**](ClusterOwner.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_cluster_time**
> update_cluster_time(ctx, cluster_time)


Set cluster time.  Time will mostly be synchronized across nodes, but there may be slight drift.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cluster_time** | [**ClusterTimeExtended**](ClusterTimeExtended.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_cluster_timezone**
> update_cluster_timezone(ctx, cluster_timezone)


Set a new timezone for the cluster.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cluster_timezone** | [**ClusterTimezoneExtended**](ClusterTimezoneExtended.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_diagnostics_gather_settings**
> update_diagnostics_gather_settings(ctx, diagnostics_gather_settings)


Set the default options for isi_gather_info.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **diagnostics_gather_settings** | [**DiagnosticsGatherSettingsExtended**](DiagnosticsGatherSettingsExtended.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_diagnostics_netlogger_settings**
> update_diagnostics_netlogger_settings(ctx, diagnostics_netlogger_settings)


Set the default options for isi_netlogger.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **diagnostics_netlogger_settings** | [**DiagnosticsNetloggerSettings**](DiagnosticsNetloggerSettings.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_timezone_settings**
> update_timezone_settings(ctx, timezone_settings)


Modify the cluster timezone.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **timezone_settings** | [**TimezoneRegionTimezone**](TimezoneRegionTimezone.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

