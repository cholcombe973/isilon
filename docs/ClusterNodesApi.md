# \ClusterNodesApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_drives_drive_add_item**](ClusterNodesApi.md#create_drives_drive_add_item) | **Post** /platform/3/cluster/nodes/{Lnn}/drives/{Driveid}/add | 
[**create_drives_drive_firmware_update_item**](ClusterNodesApi.md#create_drives_drive_firmware_update_item) | **Post** /platform/3/cluster/nodes/{Lnn}/drives/{Driveid}/firmware/update | 
[**create_drives_drive_format_item**](ClusterNodesApi.md#create_drives_drive_format_item) | **Post** /platform/3/cluster/nodes/{Lnn}/drives/{Driveid}/format | 
[**create_drives_drive_purpose_item**](ClusterNodesApi.md#create_drives_drive_purpose_item) | **Post** /platform/3/cluster/nodes/{Lnn}/drives/{Driveid}/purpose | 
[**create_drives_drive_smartfail_item**](ClusterNodesApi.md#create_drives_drive_smartfail_item) | **Post** /platform/3/cluster/nodes/{Lnn}/drives/{Driveid}/smartfail | 
[**create_drives_drive_stopfail_item**](ClusterNodesApi.md#create_drives_drive_stopfail_item) | **Post** /platform/3/cluster/nodes/{Lnn}/drives/{Driveid}/stopfail | 
[**create_drives_drive_suspend_item**](ClusterNodesApi.md#create_drives_drive_suspend_item) | **Post** /platform/3/cluster/nodes/{Lnn}/drives/{Driveid}/suspend | 
[**create_node_reboot_item**](ClusterNodesApi.md#create_node_reboot_item) | **Post** /platform/5/cluster/nodes/{Lnn}/reboot | 
[**create_node_shutdown_item**](ClusterNodesApi.md#create_node_shutdown_item) | **Post** /platform/5/cluster/nodes/{Lnn}/shutdown | 
[**get_drives_drive_firmware**](ClusterNodesApi.md#get_drives_drive_firmware) | **Get** /platform/3/cluster/nodes/{Lnn}/drives/{Driveid}/firmware | 
[**get_node_drive**](ClusterNodesApi.md#get_node_drive) | **Get** /platform/5/cluster/nodes/{Lnn}/drives/{NodeDriveId} | 
[**get_node_driveconfig**](ClusterNodesApi.md#get_node_driveconfig) | **Get** /platform/5/cluster/nodes/{Lnn}/driveconfig | 
[**get_node_drives**](ClusterNodesApi.md#get_node_drives) | **Get** /platform/5/cluster/nodes/{Lnn}/drives | 
[**get_node_drives_purposelist**](ClusterNodesApi.md#get_node_drives_purposelist) | **Get** /platform/3/cluster/nodes/{Lnn}/drives-purposelist | 
[**get_node_hardware**](ClusterNodesApi.md#get_node_hardware) | **Get** /platform/5/cluster/nodes/{Lnn}/hardware | 
[**get_node_hardware_fast**](ClusterNodesApi.md#get_node_hardware_fast) | **Get** /platform/3/cluster/nodes/{Lnn}/hardware-fast | 
[**get_node_partitions**](ClusterNodesApi.md#get_node_partitions) | **Get** /platform/3/cluster/nodes/{Lnn}/partitions | 
[**get_node_sensors**](ClusterNodesApi.md#get_node_sensors) | **Get** /platform/3/cluster/nodes/{Lnn}/sensors | 
[**get_node_sled**](ClusterNodesApi.md#get_node_sled) | **Get** /platform/5/cluster/nodes/{Lnn}/sleds/{NodeSledId} | 
[**get_node_sleds**](ClusterNodesApi.md#get_node_sleds) | **Get** /platform/5/cluster/nodes/{Lnn}/sleds | 
[**get_node_state**](ClusterNodesApi.md#get_node_state) | **Get** /platform/3/cluster/nodes/{Lnn}/state | 
[**get_node_state_readonly**](ClusterNodesApi.md#get_node_state_readonly) | **Get** /platform/3/cluster/nodes/{Lnn}/state/readonly | 
[**get_node_state_servicelight**](ClusterNodesApi.md#get_node_state_servicelight) | **Get** /platform/3/cluster/nodes/{Lnn}/state/servicelight | 
[**get_node_state_smartfail**](ClusterNodesApi.md#get_node_state_smartfail) | **Get** /platform/3/cluster/nodes/{Lnn}/state/smartfail | 
[**get_node_status**](ClusterNodesApi.md#get_node_status) | **Get** /platform/3/cluster/nodes/{Lnn}/status | 
[**get_node_status_batterystatus**](ClusterNodesApi.md#get_node_status_batterystatus) | **Get** /platform/3/cluster/nodes/{Lnn}/status/batterystatus | 
[**list_drives_drive_firmware_update**](ClusterNodesApi.md#list_drives_drive_firmware_update) | **Get** /platform/3/cluster/nodes/{Lnn}/drives/{Driveid}/firmware/update | 
[**update_node_driveconfig**](ClusterNodesApi.md#update_node_driveconfig) | **Put** /platform/5/cluster/nodes/{Lnn}/driveconfig | 
[**update_node_state_readonly**](ClusterNodesApi.md#update_node_state_readonly) | **Put** /platform/3/cluster/nodes/{Lnn}/state/readonly | 
[**update_node_state_servicelight**](ClusterNodesApi.md#update_node_state_servicelight) | **Put** /platform/3/cluster/nodes/{Lnn}/state/servicelight | 
[**update_node_state_smartfail**](ClusterNodesApi.md#update_node_state_smartfail) | **Put** /platform/3/cluster/nodes/{Lnn}/state/smartfail | 


# **create_drives_drive_add_item**
>crate::models::Empty create_drives_drive_add_item(ctx, drives_drive_add_item, lnn, driveid)


Add a drive to a node.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **drives_drive_add_item** | [**Empty**](Empty.md)|  | 
  **lnn** | **i32**|  | 
  **driveid** | **String**|  | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_drives_drive_firmware_update_item**
>crate::models::Empty create_drives_drive_firmware_update_item(ctx, drives_drive_firmware_update_item, lnn, driveid)


Start a drive firmware update.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **drives_drive_firmware_update_item** | [**DrivesDriveFirmwareUpdateItem**](DrivesDriveFirmwareUpdateItem.md)|  | 
  **lnn** | **i32**|  | 
  **driveid** | **String**|  | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_drives_drive_format_item**
>crate::models::Empty create_drives_drive_format_item(ctx, drives_drive_format_item, lnn, driveid)


Format a drive for use by OneFS.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **drives_drive_format_item** | [**DrivesDriveFormatItem**](DrivesDriveFormatItem.md)|  | 
  **lnn** | **i32**|  | 
  **driveid** | **String**|  | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_drives_drive_purpose_item**
>crate::models::Empty create_drives_drive_purpose_item(ctx, drives_drive_purpose_item, lnn, driveid)


Assign a drive to a specific use case.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **drives_drive_purpose_item** | [**DrivesDrivePurposeItem**](DrivesDrivePurposeItem.md)|  | 
  **lnn** | **i32**|  | 
  **driveid** | **String**|  | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_drives_drive_smartfail_item**
>crate::models::Empty create_drives_drive_smartfail_item(ctx, drives_drive_smartfail_item, lnn, driveid)


Remove a drive from use by OneFS.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **drives_drive_smartfail_item** | [**Empty**](Empty.md)|  | 
  **lnn** | **i32**|  | 
  **driveid** | **String**|  | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_drives_drive_stopfail_item**
>crate::models::Empty create_drives_drive_stopfail_item(ctx, drives_drive_stopfail_item, lnn, driveid)


Stop restriping from a smartfailing drive.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **drives_drive_stopfail_item** | [**Empty**](Empty.md)|  | 
  **lnn** | **i32**|  | 
  **driveid** | **String**|  | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_drives_drive_suspend_item**
>crate::models::Empty create_drives_drive_suspend_item(ctx, drives_drive_suspend_item, lnn, driveid)


Temporarily remove a drive from use by OneFS.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **drives_drive_suspend_item** | [**Empty**](Empty.md)|  | 
  **lnn** | **i32**|  | 
  **driveid** | **String**|  | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_node_reboot_item**
>crate::models::Empty create_node_reboot_item(ctx, node_reboot_item, lnn, optional)


Reboot the node specified by <LNN>.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **node_reboot_item** | [**Empty**](Empty.md)|  | 
  **lnn** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **node_reboot_item** | [**Empty**](Empty.md)|  | 
 **lnn** | **i32**|  | 
 **force** | **bool**| Force reboot on Infinity platform even if a drive sled is not present. | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_node_shutdown_item**
>crate::models::Empty create_node_shutdown_item(ctx, node_shutdown_item, lnn, optional)


Shutdown the node specified by <LNN>.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **node_shutdown_item** | [**Empty**](Empty.md)|  | 
  **lnn** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **node_shutdown_item** | [**Empty**](Empty.md)|  | 
 **lnn** | **i32**|  | 
 **force** | **bool**| Force shutdown on Infinity platform even if a drive sled is not present. | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_drives_drive_firmware**
>crate::models::DrivesDriveFirmware get_drives_drive_firmware(ctx, lnn, driveid)


Retrieve drive firmware information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **lnn** | **i32**|  | 
  **driveid** | **String**|  | 

### Return type

[**::models::DrivesDriveFirmware**](DrivesDriveFirmware.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_node_drive**
>crate::models::NodeDrives get_node_drive(ctx, node_drive_id, lnn, optional)


Retrieve drive information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **node_drive_id** | **String**| Retrieve drive information. | 
  **lnn** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **node_drive_id** | **String**| Retrieve drive information. | 
 **lnn** | **i32**|  | 
 **timeout** | **f32**| Request timeout | 

### Return type

[**::models::NodeDrives**](NodeDrives.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_node_driveconfig**
>crate::models::NodeDriveconfig get_node_driveconfig(ctx, lnn, optional)


View a node's drive subsystem XML configuration file.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **lnn** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **lnn** | **i32**|  | 
 **timeout** | **f32**| Request timeout | 

### Return type

[**::models::NodeDriveconfig**](NodeDriveconfig.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_node_drives**
>crate::models::NodeDrives get_node_drives(ctx, lnn, optional)


List the drives on this node.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **lnn** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **lnn** | **i32**|  | 
 **timeout** | **f32**| Request timeout | 

### Return type

[**::models::NodeDrives**](NodeDrives.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_node_drives_purposelist**
>crate::models::NodeDrivesPurposelist get_node_drives_purposelist(ctx, lnn)


Lists the available purposes for drives in this node.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **lnn** | **i32**|  | 

### Return type

[**::models::NodeDrivesPurposelist**](NodeDrivesPurposelist.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_node_hardware**
>crate::models::NodeHardware get_node_hardware(ctx, lnn, optional)


Retrieve node hardware identity information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **lnn** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **lnn** | **i32**|  | 
 **timeout** | **f32**| Request timeout | 

### Return type

[**::models::NodeHardware**](NodeHardware.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_node_hardware_fast**
>crate::models::NodeHardwareFast get_node_hardware_fast(ctx, lnn)


Quickly retrieve a subset of node hardware identity information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **lnn** | **i32**|  | 

### Return type

[**::models::NodeHardwareFast**](NodeHardwareFast.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_node_partitions**
>crate::models::NodePartitions get_node_partitions(ctx, lnn)


Retrieve node partition information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **lnn** | **i32**|  | 

### Return type

[**::models::NodePartitions**](NodePartitions.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_node_sensors**
>crate::models::NodeSensors get_node_sensors(ctx, lnn)


Retrieve node sensor information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **lnn** | **i32**|  | 

### Return type

[**::models::NodeSensors**](NodeSensors.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_node_sled**
>crate::models::NodeSleds get_node_sled(ctx, node_sled_id, lnn, optional)


Get detailed information for the sled specified by <SLEDID>, or all sleds in the case where <SLEDID> is 'all', in the node specified by <LNN>.  Accepts <sledid> in either 'sled' or 'all' formats.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **node_sled_id** | **String**| Get detailed information for the sled specified by &lt;SLEDID&gt;, or all sleds in the case where &lt;SLEDID&gt; is &#39;all&#39;, in the node specified by &lt;LNN&gt;.  Accepts &lt;sledid&gt; in either &#39;sled&#39; or &#39;all&#39; formats. | 
  **lnn** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **node_sled_id** | **String**| Get detailed information for the sled specified by &lt;SLEDID&gt;, or all sleds in the case where &lt;SLEDID&gt; is &#39;all&#39;, in the node specified by &lt;LNN&gt;.  Accepts &lt;sledid&gt; in either &#39;sled&#39; or &#39;all&#39; formats. | 
 **lnn** | **i32**|  | 
 **timeout** | **f32**| Request timeout | 

### Return type

[**::models::NodeSleds**](NodeSleds.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_node_sleds**
>crate::models::NodeSleds get_node_sleds(ctx, lnn, optional)


Get detailed information for all sleds in this node. Equivalent to /5/cluster/nodes/<lnn>/sleds/all.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **lnn** | **i32**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **lnn** | **i32**|  | 
 **timeout** | **f32**| Request timeout | 

### Return type

[**::models::NodeSleds**](NodeSleds.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_node_state**
>crate::models::NodeState get_node_state(ctx, lnn)


Retrieve node state information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **lnn** | **i32**|  | 

### Return type

[**::models::NodeState**](NodeState.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_node_state_readonly**
>crate::models::NodeStateReadonly get_node_state_readonly(ctx, lnn)


Retrieve node readonly state information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **lnn** | **i32**|  | 

### Return type

[**::models::NodeStateReadonly**](NodeStateReadonly.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_node_state_servicelight**
>crate::models::NodeStateServicelight get_node_state_servicelight(ctx, lnn)


Retrieve node service light state information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **lnn** | **i32**|  | 

### Return type

[**::models::NodeStateServicelight**](NodeStateServicelight.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_node_state_smartfail**
>crate::models::NodeStateSmartfail get_node_state_smartfail(ctx, lnn)


Retrieve node smartfail state information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **lnn** | **i32**|  | 

### Return type

[**::models::NodeStateSmartfail**](NodeStateSmartfail.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_node_status**
>crate::models::NodeStatus get_node_status(ctx, lnn)


Retrieve node status information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **lnn** | **i32**|  | 

### Return type

[**::models::NodeStatus**](NodeStatus.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_node_status_batterystatus**
>crate::models::NodeStatusBatterystatus get_node_status_batterystatus(ctx, lnn)


Retrieve node battery status information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **lnn** | **i32**|  | 

### Return type

[**::models::NodeStatusBatterystatus**](NodeStatusBatterystatus.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_drives_drive_firmware_update**
>crate::models::DrivesDriveFirmwareUpdate list_drives_drive_firmware_update(ctx, lnn, driveid)


Retrieve firmware update information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **lnn** | **i32**|  | 
  **driveid** | **String**|  | 

### Return type

[**::models::DrivesDriveFirmwareUpdate**](DrivesDriveFirmwareUpdate.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_node_driveconfig**
> update_node_driveconfig(ctx, node_driveconfig, lnn)


Modify a node's drive subsystem XML configuration file.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **node_driveconfig** | [**NodeDriveconfigExtended**](NodeDriveconfigExtended.md)|  | 
  **lnn** | **i32**|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_node_state_readonly**
> update_node_state_readonly(ctx, node_state_readonly, lnn)


Modify one or more node readonly state settings.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **node_state_readonly** | [**NodeStateReadonlyExtended**](NodeStateReadonlyExtended.md)|  | 
  **lnn** | **i32**|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_node_state_servicelight**
> update_node_state_servicelight(ctx, node_state_servicelight, lnn)


Modify one or more node service light state settings.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **node_state_servicelight** | [**NodeStateServicelightExtended**](NodeStateServicelightExtended.md)|  | 
  **lnn** | **i32**|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_node_state_smartfail**
> update_node_state_smartfail(ctx, node_state_smartfail, lnn)


Modify smartfail state of the node.  Only the 'smartfailed' body member has any effect on node smartfail state.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **node_state_smartfail** | [**NodeStateSmartfailExtended**](NodeStateSmartfailExtended.md)|  | 
  **lnn** | **i32**|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

