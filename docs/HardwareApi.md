# \HardwareApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_hardware_tape_name**](HardwareApi.md#create_hardware_tape_name) | **Post** /platform/3/hardware/tape/{HardwareTapeName} | 
[**delete_hardware_tape_name**](HardwareApi.md#delete_hardware_tape_name) | **Delete** /platform/3/hardware/tape/{HardwareTapeName} | 
[**get_hardware_fcport**](HardwareApi.md#get_hardware_fcport) | **Get** /platform/3/hardware/fcports/{HardwareFcportId} | 
[**get_hardware_fcports**](HardwareApi.md#get_hardware_fcports) | **Get** /platform/3/hardware/fcports | 
[**get_hardware_tapes**](HardwareApi.md#get_hardware_tapes) | **Get** /platform/3/hardware/tapes | 
[**update_hardware_fcport**](HardwareApi.md#update_hardware_fcport) | **Put** /platform/3/hardware/fcports/{HardwareFcportId} | 
[**update_hardware_tape_name**](HardwareApi.md#update_hardware_tape_name) | **Put** /platform/3/hardware/tape/{HardwareTapeName} | 


# **create_hardware_tape_name**
>crate::models::CreateHardwareTapeNameResponse create_hardware_tape_name(ctx, hardware_tape_name, hardware_tape_name2, optional)


Tape/Changer devices rescan

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **hardware_tape_name** | [**Empty**](Empty.md)|  | 
  **hardware_tape_name2** | **String**| Tape/Changer devices rescan | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **hardware_tape_name** | [**Empty**](Empty.md)|  | 
 **hardware_tape_name2** | **String**| Tape/Changer devices rescan | 
 **lnn** | **String**| Logical node number. | 
 **port** | **i32**| Scan only specified port. | 
 **timeout** | **f32**| Timeout for request. | 
 **reconcile** | **bool**| Remove entries for devices or paths that have become inaccessible. | 

### Return type

[**::models::CreateHardwareTapeNameResponse**](CreateHardwareTapeNameResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_hardware_tape_name**
> delete_hardware_tape_name(ctx, hardware_tape_name)


Tape/Changer devices remove

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **hardware_tape_name** | **String**| Tape/Changer devices remove | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_hardware_fcport**
>crate::models::HardwareFcports get_hardware_fcport(ctx, hardware_fcport_id, optional)


Get one fibre-channel port

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **hardware_fcport_id** | **i32**| Get one fibre-channel port | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **hardware_fcport_id** | **i32**| Get one fibre-channel port | 
 **lnn** | **String**| Logical node number. | 

### Return type

[**::models::HardwareFcports**](HardwareFcports.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_hardware_fcports**
>crate::models::HardwareFcports get_hardware_fcports(ctx, optional)


Get list of fibre-channel ports

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **lnn** | **String**| Logical node number. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::HardwareFcports**](HardwareFcports.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_hardware_tapes**
>crate::models::HardwareTapes get_hardware_tapes(ctx, optional)


Get list Tape and Changer devices

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **node** | **String**| List only devices on the specified node. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 
 **devname** | **String**| List only the named device. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **activepath** | **String**| List devices with only active paths. | 
 **_type** | **String**| Restrict to list on tape or mc device. | 

### Return type

[**::models::HardwareTapes**](HardwareTapes.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_hardware_fcport**
> update_hardware_fcport(ctx, hardware_fcport, hardware_fcport_id, optional)


Change wwnn, wwpn, state, topology, or rate of a FC port

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **hardware_fcport** | [**HardwareFcport**](HardwareFcport.md)|  | 
  **hardware_fcport_id** | **i32**| Change wwnn, wwpn, state, topology, or rate of a FC port | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **hardware_fcport** | [**HardwareFcport**](HardwareFcport.md)|  | 
 **hardware_fcport_id** | **i32**| Change wwnn, wwpn, state, topology, or rate of a FC port | 
 **lnn** | **String**| Logical node number. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_hardware_tape_name**
> update_hardware_tape_name(ctx, hardware_tape_name_params, hardware_tape_name)


Tape/Changer device modify

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **hardware_tape_name_params** | [**HardwareTapeNameParams**](HardwareTapeNameParams.md)|  | 
  **hardware_tape_name** | **String**| Tape/Changer device modify | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

