# \NetworkApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_dnscache_flush_item**](NetworkApi.md#create_dnscache_flush_item) | **Post** /platform/3/network/dnscache/flush | 
[**create_network_groupnet**](NetworkApi.md#create_network_groupnet) | **Post** /platform/3/network/groupnets | 
[**create_network_sc_rebalance_all_item**](NetworkApi.md#create_network_sc_rebalance_all_item) | **Post** /platform/3/network/sc-rebalance-all | 
[**delete_network_groupnet**](NetworkApi.md#delete_network_groupnet) | **Delete** /platform/3/network/groupnets/{NetworkGroupnetId} | 
[**get_network_dnscache**](NetworkApi.md#get_network_dnscache) | **Get** /platform/3/network/dnscache | 
[**get_network_external**](NetworkApi.md#get_network_external) | **Get** /platform/3/network/external | 
[**get_network_groupnet**](NetworkApi.md#get_network_groupnet) | **Get** /platform/3/network/groupnets/{NetworkGroupnetId} | 
[**get_network_interfaces**](NetworkApi.md#get_network_interfaces) | **Get** /platform/4/network/interfaces | 
[**get_network_pools**](NetworkApi.md#get_network_pools) | **Get** /platform/3/network/pools | 
[**get_network_rules**](NetworkApi.md#get_network_rules) | **Get** /platform/3/network/rules | 
[**get_network_subnets**](NetworkApi.md#get_network_subnets) | **Get** /platform/4/network/subnets | 
[**list_network_groupnets**](NetworkApi.md#list_network_groupnets) | **Get** /platform/3/network/groupnets | 
[**update_network_dnscache**](NetworkApi.md#update_network_dnscache) | **Put** /platform/3/network/dnscache | 
[**update_network_external**](NetworkApi.md#update_network_external) | **Put** /platform/3/network/external | 
[**update_network_groupnet**](NetworkApi.md#update_network_groupnet) | **Put** /platform/3/network/groupnets/{NetworkGroupnetId} | 


# **create_dnscache_flush_item**
> ::models::Empty create_dnscache_flush_item(ctx, dnscache_flush_item)


Flush the DNSCache.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **dnscache_flush_item** | [**Empty**](Empty.md)|  | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_network_groupnet**
> ::models::CreateResponse create_network_groupnet(ctx, network_groupnet)


Create a new groupnet.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **network_groupnet** | [**NetworkGroupnetCreateParams**](NetworkGroupnetCreateParams.md)|  | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_network_sc_rebalance_all_item**
> ::models::Empty create_network_sc_rebalance_all_item(ctx, network_sc_rebalance_all_item)


Rebalance IP addresses in all pools.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **network_sc_rebalance_all_item** | [**Empty**](Empty.md)|  | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_network_groupnet**
> delete_network_groupnet(ctx, network_groupnet_id)


Delete a network groupnet.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **network_groupnet_id** | **String**| Delete a network groupnet. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_network_dnscache**
> ::models::NetworkDnscache get_network_dnscache(ctx, )


View network dns cache settings.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::NetworkDnscache**](NetworkDnscache.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_network_external**
> ::models::NetworkExternal get_network_external(ctx, )


View external network settings.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::NetworkExternal**](NetworkExternal.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_network_groupnet**
> ::models::NetworkGroupnets get_network_groupnet(ctx, network_groupnet_id)


View a network groupnet.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **network_groupnet_id** | **String**| View a network groupnet. | 

### Return type

[**::models::NetworkGroupnets**](NetworkGroupnets.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_network_interfaces**
> ::models::PoolsPoolInterfaces get_network_interfaces(ctx, optional)


Get a list of interfaces.

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
 **network** | **String**| Show interfaces associated with external and/or internal networks. Default is &#39;external&#39; | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 
 **lnns** | **String**| Get a list of interfaces for the specified lnn. | 
 **alloc_method** | **String**| Filter addresses and owners by pool address allocation method. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **dir** | **String**| The direction of the sort. | 

### Return type

[**::models::PoolsPoolInterfaces**](PoolsPoolInterfaces.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_network_pools**
> ::models::NetworkPools get_network_pools(ctx, optional)


Get a list of flexnet pools.

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
 **subnet** | **String**| If specified, only pools for this subnet will be returned. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 
 **access_zone** | **String**| If specified, only pools with this zone name will be returned. | 
 **alloc_method** | **String**| If specified, only pools with this allocation type will be returned. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **groupnet** | **String**| If specified, only pools for this groupnet will be returned. | 
 **dir** | **String**| The direction of the sort. | 

### Return type

[**::models::NetworkPools**](NetworkPools.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_network_rules**
> ::models::PoolsPoolRulesExtended get_network_rules(ctx, optional)


Get a list of network rules.

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
 **subnet** | **String**| Name of the subnet to list rules from. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **dir** | **String**| The direction of the sort. | 
 **groupnet** | **String**| Name of the groupnet to list rules from. | 
 **pool** | **String**| Name of the pool to list rules from. | 

### Return type

[**::models::PoolsPoolRulesExtended**](PoolsPoolRulesExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_network_subnets**
> ::models::GroupnetSubnetsExtended get_network_subnets(ctx, optional)


Get a list of subnets.

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
 **groupnet** | **String**| If specified, only subnets for this groupnet will be returned. | 
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

# **list_network_groupnets**
> ::models::NetworkGroupnetsExtended list_network_groupnets(ctx, optional)


Get a list of groupnets.

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
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **dir** | **String**| The direction of the sort. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::NetworkGroupnetsExtended**](NetworkGroupnetsExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_network_dnscache**
> update_network_dnscache(ctx, network_dnscache)


Modify network dns cache settings.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **network_dnscache** | [**NetworkDnscacheExtended**](NetworkDnscacheExtended.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_network_external**
> update_network_external(ctx, network_external)


Modify external network settings.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **network_external** | [**NetworkExternalExtended**](NetworkExternalExtended.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_network_groupnet**
> update_network_groupnet(ctx, network_groupnet, network_groupnet_id)


Modify a network groupnet.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **network_groupnet** | [**NetworkGroupnet**](NetworkGroupnet.md)|  | 
  **network_groupnet_id** | **String**| Modify a network groupnet. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

