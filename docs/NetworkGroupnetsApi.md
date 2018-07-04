# \NetworkGroupnetsApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_groupnet_subnet**](NetworkGroupnetsApi.md#create_groupnet_subnet) | **Post** /platform/4/network/groupnets/{Groupnet}/subnets | 
[**create_subnets_subnet_pool**](NetworkGroupnetsApi.md#create_subnets_subnet_pool) | **Post** /platform/3/network/groupnets/{Groupnet}/subnets/{Subnet}/pools | 
[**delete_groupnet_subnet**](NetworkGroupnetsApi.md#delete_groupnet_subnet) | **Delete** /platform/4/network/groupnets/{Groupnet}/subnets/{GroupnetSubnetId} | 
[**delete_subnets_subnet_pool**](NetworkGroupnetsApi.md#delete_subnets_subnet_pool) | **Delete** /platform/3/network/groupnets/{Groupnet}/subnets/{Subnet}/pools/{SubnetsSubnetPoolId} | 
[**get_groupnet_subnet**](NetworkGroupnetsApi.md#get_groupnet_subnet) | **Get** /platform/4/network/groupnets/{Groupnet}/subnets/{GroupnetSubnetId} | 
[**get_subnets_subnet_pool**](NetworkGroupnetsApi.md#get_subnets_subnet_pool) | **Get** /platform/3/network/groupnets/{Groupnet}/subnets/{Subnet}/pools/{SubnetsSubnetPoolId} | 
[**list_groupnet_subnets**](NetworkGroupnetsApi.md#list_groupnet_subnets) | **Get** /platform/4/network/groupnets/{Groupnet}/subnets | 
[**list_subnets_subnet_pools**](NetworkGroupnetsApi.md#list_subnets_subnet_pools) | **Get** /platform/3/network/groupnets/{Groupnet}/subnets/{Subnet}/pools | 
[**update_groupnet_subnet**](NetworkGroupnetsApi.md#update_groupnet_subnet) | **Put** /platform/4/network/groupnets/{Groupnet}/subnets/{GroupnetSubnetId} | 
[**update_subnets_subnet_pool**](NetworkGroupnetsApi.md#update_subnets_subnet_pool) | **Put** /platform/3/network/groupnets/{Groupnet}/subnets/{Subnet}/pools/{SubnetsSubnetPoolId} | 


# **create_groupnet_subnet**
> ::models::CreateResponse create_groupnet_subnet(ctx, groupnet_subnet, groupnet)


Create a new subnet.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **groupnet_subnet** | [**GroupnetSubnetCreateParams**](GroupnetSubnetCreateParams.md)|  | 
  **groupnet** | **String**|  | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_subnets_subnet_pool**
> ::models::CreateResponse create_subnets_subnet_pool(ctx, subnets_subnet_pool, groupnet, subnet, optional)


Create a new pool.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **subnets_subnet_pool** | [**SubnetsSubnetPoolCreateParams**](SubnetsSubnetPoolCreateParams.md)|  | 
  **groupnet** | **String**|  | 
  **subnet** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **subnets_subnet_pool** | [**SubnetsSubnetPoolCreateParams**](SubnetsSubnetPoolCreateParams.md)|  | 
 **groupnet** | **String**|  | 
 **subnet** | **String**|  | 
 **force** | **bool**| Force creating this pool even if it causes an MTU conflict. | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_groupnet_subnet**
> delete_groupnet_subnet(ctx, groupnet_subnet_id, groupnet, optional)


Delete a network subnet..

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **groupnet_subnet_id** | **String**| Delete a network subnet.. | 
  **groupnet** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **groupnet_subnet_id** | **String**| Delete a network subnet.. | 
 **groupnet** | **String**|  | 
 **force** | **bool**| force deleting this subnet even if pools in other subnets rely on this subnet&#39;s SC VIP. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_subnets_subnet_pool**
> delete_subnets_subnet_pool(ctx, subnets_subnet_pool_id, groupnet, subnet)


Delete a network pool.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **subnets_subnet_pool_id** | **String**| Delete a network pool. | 
  **groupnet** | **String**|  | 
  **subnet** | **String**|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_groupnet_subnet**
> ::models::GroupnetSubnets get_groupnet_subnet(ctx, groupnet_subnet_id, groupnet)


View a network subnet.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **groupnet_subnet_id** | **String**| View a network subnet. | 
  **groupnet** | **String**|  | 

### Return type

[**::models::GroupnetSubnets**](GroupnetSubnets.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_subnets_subnet_pool**
> ::models::SubnetsSubnetPools get_subnets_subnet_pool(ctx, subnets_subnet_pool_id, groupnet, subnet)


View a single network pool.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **subnets_subnet_pool_id** | **String**| View a single network pool. | 
  **groupnet** | **String**|  | 
  **subnet** | **String**|  | 

### Return type

[**::models::SubnetsSubnetPools**](SubnetsSubnetPools.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_groupnet_subnets**
> ::models::GroupnetSubnetsExtended list_groupnet_subnets(ctx, groupnet, optional)


Get a list of subnets.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **groupnet** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **groupnet** | **String**|  | 
 **sort** | **String**| The field that will be used for sorting. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **dir** | **String**| The direction of the sort. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::GroupnetSubnetsExtended**](GroupnetSubnetsExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_subnets_subnet_pools**
> ::models::SubnetsSubnetPoolsExtended list_subnets_subnet_pools(ctx, groupnet, subnet, optional)


Get a list of network pools.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **groupnet** | **String**|  | 
  **subnet** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **groupnet** | **String**|  | 
 **subnet** | **String**|  | 
 **sort** | **String**| The field that will be used for sorting. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 
 **access_zone** | **String**| If specified, only pools with this zone name will be returned. | 
 **alloc_method** | **String**| If specified, only pools with this allocation type will be returned. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **dir** | **String**| The direction of the sort. | 

### Return type

[**::models::SubnetsSubnetPoolsExtended**](SubnetsSubnetPoolsExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_groupnet_subnet**
> update_groupnet_subnet(ctx, groupnet_subnet, groupnet_subnet_id, groupnet, optional)


Modify a network subnet.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **groupnet_subnet** | [**GroupnetSubnet**](GroupnetSubnet.md)|  | 
  **groupnet_subnet_id** | **String**| Modify a network subnet. | 
  **groupnet** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **groupnet_subnet** | [**GroupnetSubnet**](GroupnetSubnet.md)|  | 
 **groupnet_subnet_id** | **String**| Modify a network subnet. | 
 **groupnet** | **String**|  | 
 **force** | **bool**| force modifying this subnet even if it causes an MTU conflict. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_subnets_subnet_pool**
> update_subnets_subnet_pool(ctx, subnets_subnet_pool, subnets_subnet_pool_id, groupnet, subnet, optional)


Modify a network pool.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **subnets_subnet_pool** | [**SubnetsSubnetPool**](SubnetsSubnetPool.md)|  | 
  **subnets_subnet_pool_id** | **String**| Modify a network pool. | 
  **groupnet** | **String**|  | 
  **subnet** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **subnets_subnet_pool** | [**SubnetsSubnetPool**](SubnetsSubnetPool.md)|  | 
 **subnets_subnet_pool_id** | **String**| Modify a network pool. | 
 **groupnet** | **String**|  | 
 **subnet** | **String**|  | 
 **force** | **bool**| force creating this pool even if it causes an MTU conflict. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

