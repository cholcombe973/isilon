# \StoragepoolApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_compatibilities_class_active_item**](StoragepoolApi.md#create_compatibilities_class_active_item) | **Post** /platform/1/storagepool/compatibilities/class/active | 
[**create_compatibilities_ssd_active_item**](StoragepoolApi.md#create_compatibilities_ssd_active_item) | **Post** /platform/3/storagepool/compatibilities/ssd/active | 
[**create_storagepool_nodepool**](StoragepoolApi.md#create_storagepool_nodepool) | **Post** /platform/3/storagepool/nodepools | 
[**create_storagepool_tier**](StoragepoolApi.md#create_storagepool_tier) | **Post** /platform/1/storagepool/tiers | 
[**delete_compatibilities_class_active_by_id**](StoragepoolApi.md#delete_compatibilities_class_active_by_id) | **Delete** /platform/1/storagepool/compatibilities/class/active/{CompatibilitiesClassActiveId} | 
[**delete_compatibilities_ssd_active_by_id**](StoragepoolApi.md#delete_compatibilities_ssd_active_by_id) | **Delete** /platform/3/storagepool/compatibilities/ssd/active/{CompatibilitiesSsdActiveId} | 
[**delete_storagepool_nodepool**](StoragepoolApi.md#delete_storagepool_nodepool) | **Delete** /platform/3/storagepool/nodepools/{StoragepoolNodepoolId} | 
[**delete_storagepool_nodepools**](StoragepoolApi.md#delete_storagepool_nodepools) | **Delete** /platform/3/storagepool/nodepools | 
[**delete_storagepool_tier**](StoragepoolApi.md#delete_storagepool_tier) | **Delete** /platform/1/storagepool/tiers/{StoragepoolTierId} | 
[**delete_storagepool_tiers**](StoragepoolApi.md#delete_storagepool_tiers) | **Delete** /platform/1/storagepool/tiers | 
[**get_compatibilities_class_active_by_id**](StoragepoolApi.md#get_compatibilities_class_active_by_id) | **Get** /platform/1/storagepool/compatibilities/class/active/{CompatibilitiesClassActiveId} | 
[**get_compatibilities_class_available**](StoragepoolApi.md#get_compatibilities_class_available) | **Get** /platform/1/storagepool/compatibilities/class/available | 
[**get_compatibilities_ssd_active_by_id**](StoragepoolApi.md#get_compatibilities_ssd_active_by_id) | **Get** /platform/3/storagepool/compatibilities/ssd/active/{CompatibilitiesSsdActiveId} | 
[**get_compatibilities_ssd_available**](StoragepoolApi.md#get_compatibilities_ssd_available) | **Get** /platform/1/storagepool/compatibilities/ssd/available | 
[**get_storagepool_nodepool**](StoragepoolApi.md#get_storagepool_nodepool) | **Get** /platform/3/storagepool/nodepools/{StoragepoolNodepoolId} | 
[**get_storagepool_settings**](StoragepoolApi.md#get_storagepool_settings) | **Get** /platform/5/storagepool/settings | 
[**get_storagepool_status**](StoragepoolApi.md#get_storagepool_status) | **Get** /platform/1/storagepool/status | 
[**get_storagepool_storagepools**](StoragepoolApi.md#get_storagepool_storagepools) | **Get** /platform/3/storagepool/storagepools | 
[**get_storagepool_suggested_protection_nid**](StoragepoolApi.md#get_storagepool_suggested_protection_nid) | **Get** /platform/3/storagepool/suggested-protection/{StoragepoolSuggestedProtectionNid} | 
[**get_storagepool_tier**](StoragepoolApi.md#get_storagepool_tier) | **Get** /platform/1/storagepool/tiers/{StoragepoolTierId} | 
[**get_storagepool_unprovisioned**](StoragepoolApi.md#get_storagepool_unprovisioned) | **Get** /platform/1/storagepool/unprovisioned | 
[**list_compatibilities_class_active**](StoragepoolApi.md#list_compatibilities_class_active) | **Get** /platform/1/storagepool/compatibilities/class/active | 
[**list_compatibilities_ssd_active**](StoragepoolApi.md#list_compatibilities_ssd_active) | **Get** /platform/3/storagepool/compatibilities/ssd/active | 
[**list_storagepool_nodepools**](StoragepoolApi.md#list_storagepool_nodepools) | **Get** /platform/3/storagepool/nodepools | 
[**list_storagepool_tiers**](StoragepoolApi.md#list_storagepool_tiers) | **Get** /platform/1/storagepool/tiers | 
[**update_compatibilities_ssd_active_by_id**](StoragepoolApi.md#update_compatibilities_ssd_active_by_id) | **Put** /platform/3/storagepool/compatibilities/ssd/active/{CompatibilitiesSsdActiveId} | 
[**update_storagepool_nodepool**](StoragepoolApi.md#update_storagepool_nodepool) | **Put** /platform/3/storagepool/nodepools/{StoragepoolNodepoolId} | 
[**update_storagepool_settings**](StoragepoolApi.md#update_storagepool_settings) | **Put** /platform/5/storagepool/settings | 
[**update_storagepool_tier**](StoragepoolApi.md#update_storagepool_tier) | **Put** /platform/1/storagepool/tiers/{StoragepoolTierId} | 


# **create_compatibilities_class_active_item**
> ::models::CreateCompatibilitiesClassActiveItemResponse create_compatibilities_class_active_item(ctx, compatibilities_class_active_item, optional)


Create a new compatibility

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **compatibilities_class_active_item** | [**CompatibilitiesClassActiveItem**](CompatibilitiesClassActiveItem.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **compatibilities_class_active_item** | [**CompatibilitiesClassActiveItem**](CompatibilitiesClassActiveItem.md)|  | 
 **assess** | **bool**| Do not perform action, only test that it is possible. | 

### Return type

[**::models::CreateCompatibilitiesClassActiveItemResponse**](CreateCompatibilitiesClassActiveItemResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_compatibilities_ssd_active_item**
> ::models::CreateCompatibilitiesClassActiveItemResponse create_compatibilities_ssd_active_item(ctx, compatibilities_ssd_active_item, optional)


Create a new ssd compatibility

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **compatibilities_ssd_active_item** | [**CompatibilitiesSsdActiveItem**](CompatibilitiesSsdActiveItem.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **compatibilities_ssd_active_item** | [**CompatibilitiesSsdActiveItem**](CompatibilitiesSsdActiveItem.md)|  | 
 **assess** | **bool**| Do not perform action, only test that it is possible. | 

### Return type

[**::models::CreateCompatibilitiesClassActiveItemResponse**](CreateCompatibilitiesClassActiveItemResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_storagepool_nodepool**
> ::models::CreateStoragepoolTierResponse create_storagepool_nodepool(ctx, storagepool_nodepool)


Create a new node pool.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **storagepool_nodepool** | [**StoragepoolNodepoolCreateParams**](StoragepoolNodepoolCreateParams.md)|  | 

### Return type

[**::models::CreateStoragepoolTierResponse**](CreateStoragepoolTierResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_storagepool_tier**
> ::models::CreateStoragepoolTierResponse create_storagepool_tier(ctx, storagepool_tier)


Create a new tier.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **storagepool_tier** | [**StoragepoolTierCreateParams**](StoragepoolTierCreateParams.md)|  | 

### Return type

[**::models::CreateStoragepoolTierResponse**](CreateStoragepoolTierResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_compatibilities_class_active_by_id**
> delete_compatibilities_class_active_by_id(ctx, compatibilities_class_active_id, optional)


Delete an active compatibility by id

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **compatibilities_class_active_id** | **String**| Delete an active compatibility by id | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **compatibilities_class_active_id** | **String**| Delete an active compatibility by id | 
 **assess** | **bool**| Do not perform action, only test that it is possible. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_compatibilities_ssd_active_by_id**
> delete_compatibilities_ssd_active_by_id(ctx, compatibilities_ssd_active_id, optional)


Delete an active ssd compatibility by id

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **compatibilities_ssd_active_id** | **String**| Delete an active ssd compatibility by id | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **compatibilities_ssd_active_id** | **String**| Delete an active ssd compatibility by id | 
 **assess** | **bool**| Do not perform action, only test that it is possible. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_storagepool_nodepool**
> delete_storagepool_nodepool(ctx, storagepool_nodepool_id)


Delete node pool.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **storagepool_nodepool_id** | **String**| Delete node pool. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_storagepool_nodepools**
> delete_storagepool_nodepools(ctx, )


Delete all node pools.

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

# **delete_storagepool_tier**
> delete_storagepool_tier(ctx, storagepool_tier_id)


Delete tier.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **storagepool_tier_id** | **String**| Delete tier. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_storagepool_tiers**
> delete_storagepool_tiers(ctx, )


Delete all tiers.

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

# **get_compatibilities_class_active_by_id**
> ::models::CompatibilitiesClassActive get_compatibilities_class_active_by_id(ctx, compatibilities_class_active_id)


Get an active compatibilities by id

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **compatibilities_class_active_id** | **String**| Get an active compatibilities by id | 

### Return type

[**::models::CompatibilitiesClassActive**](CompatibilitiesClassActive.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_compatibilities_class_available**
> ::models::CompatibilitiesClassAvailable get_compatibilities_class_available(ctx, )


Get a list of available compatibilities

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::CompatibilitiesClassAvailable**](CompatibilitiesClassAvailable.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_compatibilities_ssd_active_by_id**
> ::models::CompatibilitiesSsdActive get_compatibilities_ssd_active_by_id(ctx, compatibilities_ssd_active_id)


Get a active ssd compatibilities by id

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **compatibilities_ssd_active_id** | **String**| Get a active ssd compatibilities by id | 

### Return type

[**::models::CompatibilitiesSsdActive**](CompatibilitiesSsdActive.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_compatibilities_ssd_available**
> ::models::CompatibilitiesSsdAvailable get_compatibilities_ssd_available(ctx, )


Get a list of available ssd compatibilities

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::CompatibilitiesSsdAvailable**](CompatibilitiesSsdAvailable.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_storagepool_nodepool**
> ::models::StoragepoolNodepools get_storagepool_nodepool(ctx, storagepool_nodepool_id)


Retrieve node pool information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **storagepool_nodepool_id** | **String**| Retrieve node pool information. | 

### Return type

[**::models::StoragepoolNodepools**](StoragepoolNodepools.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_storagepool_settings**
> ::models::StoragepoolSettings get_storagepool_settings(ctx, )


List all storagepool settings.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::StoragepoolSettings**](StoragepoolSettings.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_storagepool_status**
> ::models::StoragepoolStatus get_storagepool_status(ctx, )


List any health conditions detected.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::StoragepoolStatus**](StoragepoolStatus.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_storagepool_storagepools**
> ::models::StoragepoolStoragepools get_storagepool_storagepools(ctx, optional)


List all storage pools.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sort** | **String**| The field that will be used for sorting. | 
 **toplevels** | **String**| If true, node pools contained within tiers will be filtered out of results. | 
 **dir** | **String**| The direction of the sort. | 

### Return type

[**::models::StoragepoolStoragepools**](StoragepoolStoragepools.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_storagepool_suggested_protection_nid**
> ::models::StoragepoolSuggestedProtection get_storagepool_suggested_protection_nid(ctx, storagepool_suggested_protection_nid)


Retrieve the suggested protection policy.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **storagepool_suggested_protection_nid** | **String**| Retrieve the suggested protection policy. | 

### Return type

[**::models::StoragepoolSuggestedProtection**](StoragepoolSuggestedProtection.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_storagepool_tier**
> ::models::StoragepoolTiers get_storagepool_tier(ctx, storagepool_tier_id)


Retrieve tier information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **storagepool_tier_id** | **String**| Retrieve tier information. | 

### Return type

[**::models::StoragepoolTiers**](StoragepoolTiers.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_storagepool_unprovisioned**
> ::models::StoragepoolUnprovisioned get_storagepool_unprovisioned(ctx, )


Get the unprovisioned nodes and drives

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::StoragepoolUnprovisioned**](StoragepoolUnprovisioned.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_compatibilities_class_active**
> ::models::CompatibilitiesClassActiveExtended list_compatibilities_class_active(ctx, )


Get a list of active compatibilities

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::CompatibilitiesClassActiveExtended**](CompatibilitiesClassActiveExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_compatibilities_ssd_active**
> ::models::CompatibilitiesSsdActiveExtended list_compatibilities_ssd_active(ctx, )


Get a list of active ssd compatibilities

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::CompatibilitiesSsdActiveExtended**](CompatibilitiesSsdActiveExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_storagepool_nodepools**
> ::models::StoragepoolNodepoolsExtended list_storagepool_nodepools(ctx, )


List all node pools.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::StoragepoolNodepoolsExtended**](StoragepoolNodepoolsExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_storagepool_tiers**
> ::models::StoragepoolTiersExtended list_storagepool_tiers(ctx, )


List all tiers.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::StoragepoolTiersExtended**](StoragepoolTiersExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_compatibilities_ssd_active_by_id**
> update_compatibilities_ssd_active_by_id(ctx, compatibilities_ssd_active_id_params, compatibilities_ssd_active_id, optional)


Modify an ssd compatibility by id

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **compatibilities_ssd_active_id_params** | [**CompatibilitiesSsdActiveIdParams**](CompatibilitiesSsdActiveIdParams.md)|  | 
  **compatibilities_ssd_active_id** | **String**| Modify an ssd compatibility by id | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **compatibilities_ssd_active_id_params** | [**CompatibilitiesSsdActiveIdParams**](CompatibilitiesSsdActiveIdParams.md)|  | 
 **compatibilities_ssd_active_id** | **String**| Modify an ssd compatibility by id | 
 **assess** | **bool**| Do not perform action, only test that it is possible. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_storagepool_nodepool**
> update_storagepool_nodepool(ctx, storagepool_nodepool, storagepool_nodepool_id)


Modify node pool. All input fields are optional, but one or more must be supplied.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **storagepool_nodepool** | [**StoragepoolNodepool**](StoragepoolNodepool.md)|  | 
  **storagepool_nodepool_id** | **String**| Modify node pool. All input fields are optional, but one or more must be supplied. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_storagepool_settings**
> update_storagepool_settings(ctx, storagepool_settings)


Modify one or more settings.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **storagepool_settings** | [**StoragepoolSettingsExtended**](StoragepoolSettingsExtended.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_storagepool_tier**
> update_storagepool_tier(ctx, storagepool_tier, storagepool_tier_id)


Modify tier. All input fields are optional, but one or more must be supplied.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **storagepool_tier** | [**StoragepoolTier**](StoragepoolTier.md)|  | 
  **storagepool_tier_id** | **String**| Modify tier. All input fields are optional, but one or more must be supplied. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

