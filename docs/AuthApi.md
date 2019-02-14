# \AuthApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_auth_cache_item**](AuthApi.md#create_auth_cache_item) | **Post** /platform/4/auth/cache | 
[**create_auth_group**](AuthApi.md#create_auth_group) | **Post** /platform/1/auth/groups | 
[**create_auth_refresh_item**](AuthApi.md#create_auth_refresh_item) | **Post** /platform/3/auth/refresh | 
[**create_auth_role**](AuthApi.md#create_auth_role) | **Post** /platform/1/auth/roles | 
[**create_auth_user**](AuthApi.md#create_auth_user) | **Post** /platform/1/auth/users | 
[**create_mapping_identity**](AuthApi.md#create_mapping_identity) | **Post** /platform/1/auth/mapping/identities | 
[**create_mapping_identity_0**](AuthApi.md#create_mapping_identity_0) | **Post** /platform/1/auth/mapping/identities/{MappingIdentityId} | 
[**create_providers_ads_item**](AuthApi.md#create_providers_ads_item) | **Post** /platform/3/auth/providers/ads | 
[**create_providers_file_item**](AuthApi.md#create_providers_file_item) | **Post** /platform/1/auth/providers/file | 
[**create_providers_krb5_item**](AuthApi.md#create_providers_krb5_item) | **Post** /platform/3/auth/providers/krb5 | 
[**create_providers_ldap_item**](AuthApi.md#create_providers_ldap_item) | **Post** /platform/4/auth/providers/ldap | 
[**create_providers_nis_item**](AuthApi.md#create_providers_nis_item) | **Post** /platform/3/auth/providers/nis | 
[**create_settings_krb5_domain**](AuthApi.md#create_settings_krb5_domain) | **Post** /platform/1/auth/settings/krb5/domains | 
[**create_settings_krb5_realm**](AuthApi.md#create_settings_krb5_realm) | **Post** /platform/1/auth/settings/krb5/realms | 
[**delete_auth_group**](AuthApi.md#delete_auth_group) | **Delete** /platform/1/auth/groups/{AuthGroupId} | 
[**delete_auth_groups**](AuthApi.md#delete_auth_groups) | **Delete** /platform/1/auth/groups | 
[**delete_auth_role**](AuthApi.md#delete_auth_role) | **Delete** /platform/1/auth/roles/{AuthRoleId} | 
[**delete_auth_user**](AuthApi.md#delete_auth_user) | **Delete** /platform/1/auth/users/{AuthUserId} | 
[**delete_auth_users**](AuthApi.md#delete_auth_users) | **Delete** /platform/1/auth/users | 
[**delete_mapping_identities**](AuthApi.md#delete_mapping_identities) | **Delete** /platform/1/auth/mapping/identities | 
[**delete_mapping_identity**](AuthApi.md#delete_mapping_identity) | **Delete** /platform/1/auth/mapping/identities/{MappingIdentityId} | 
[**delete_providers_ads_by_id**](AuthApi.md#delete_providers_ads_by_id) | **Delete** /platform/3/auth/providers/ads/{ProvidersAdsId} | 
[**delete_providers_file_by_id**](AuthApi.md#delete_providers_file_by_id) | **Delete** /platform/1/auth/providers/file/{ProvidersFileId} | 
[**delete_providers_krb5_by_id**](AuthApi.md#delete_providers_krb5_by_id) | **Delete** /platform/3/auth/providers/krb5/{ProvidersKrb5Id} | 
[**delete_providers_ldap_by_id**](AuthApi.md#delete_providers_ldap_by_id) | **Delete** /platform/4/auth/providers/ldap/{ProvidersLdapId} | 
[**delete_providers_local_by_id**](AuthApi.md#delete_providers_local_by_id) | **Delete** /platform/1/auth/providers/local/{ProvidersLocalId} | 
[**delete_providers_nis_by_id**](AuthApi.md#delete_providers_nis_by_id) | **Delete** /platform/3/auth/providers/nis/{ProvidersNisId} | 
[**delete_settings_krb5_domain**](AuthApi.md#delete_settings_krb5_domain) | **Delete** /platform/1/auth/settings/krb5/domains/{SettingsKrb5DomainId} | 
[**delete_settings_krb5_realm**](AuthApi.md#delete_settings_krb5_realm) | **Delete** /platform/1/auth/settings/krb5/realms/{SettingsKrb5RealmId} | 
[**get_auth_access_user**](AuthApi.md#get_auth_access_user) | **Get** /platform/1/auth/access/{AuthAccessUser} | 
[**get_auth_group**](AuthApi.md#get_auth_group) | **Get** /platform/1/auth/groups/{AuthGroupId} | 
[**get_auth_id**](AuthApi.md#get_auth_id) | **Get** /platform/1/auth/id | 
[**get_auth_ldap_template**](AuthApi.md#get_auth_ldap_template) | **Get** /platform/4/auth/ldap-templates/{AuthLdapTemplateId} | 
[**get_auth_ldap_templates**](AuthApi.md#get_auth_ldap_templates) | **Get** /platform/4/auth/ldap-templates | 
[**get_auth_log_level**](AuthApi.md#get_auth_log_level) | **Get** /platform/3/auth/log-level | 
[**get_auth_netgroup**](AuthApi.md#get_auth_netgroup) | **Get** /platform/1/auth/netgroups/{AuthNetgroupId} | 
[**get_auth_privileges**](AuthApi.md#get_auth_privileges) | **Get** /platform/1/auth/privileges | 
[**get_auth_role**](AuthApi.md#get_auth_role) | **Get** /platform/1/auth/roles/{AuthRoleId} | 
[**get_auth_shells**](AuthApi.md#get_auth_shells) | **Get** /platform/1/auth/shells | 
[**get_auth_user**](AuthApi.md#get_auth_user) | **Get** /platform/1/auth/users/{AuthUserId} | 
[**get_auth_wellknown**](AuthApi.md#get_auth_wellknown) | **Get** /platform/1/auth/wellknowns/{AuthWellknownId} | 
[**get_auth_wellknowns**](AuthApi.md#get_auth_wellknowns) | **Get** /platform/1/auth/wellknowns | 
[**get_mapping_dump**](AuthApi.md#get_mapping_dump) | **Get** /platform/3/auth/mapping/dump | 
[**get_mapping_identity**](AuthApi.md#get_mapping_identity) | **Get** /platform/1/auth/mapping/identities/{MappingIdentityId} | 
[**get_mapping_users_lookup**](AuthApi.md#get_mapping_users_lookup) | **Get** /platform/1/auth/mapping/users/lookup | 
[**get_mapping_users_rules**](AuthApi.md#get_mapping_users_rules) | **Get** /platform/1/auth/mapping/users/rules | 
[**get_providers_ads_by_id**](AuthApi.md#get_providers_ads_by_id) | **Get** /platform/3/auth/providers/ads/{ProvidersAdsId} | 
[**get_providers_file_by_id**](AuthApi.md#get_providers_file_by_id) | **Get** /platform/1/auth/providers/file/{ProvidersFileId} | 
[**get_providers_krb5_by_id**](AuthApi.md#get_providers_krb5_by_id) | **Get** /platform/3/auth/providers/krb5/{ProvidersKrb5Id} | 
[**get_providers_ldap_by_id**](AuthApi.md#get_providers_ldap_by_id) | **Get** /platform/4/auth/providers/ldap/{ProvidersLdapId} | 
[**get_providers_local**](AuthApi.md#get_providers_local) | **Get** /platform/1/auth/providers/local | 
[**get_providers_local_by_id**](AuthApi.md#get_providers_local_by_id) | **Get** /platform/1/auth/providers/local/{ProvidersLocalId} | 
[**get_providers_nis_by_id**](AuthApi.md#get_providers_nis_by_id) | **Get** /platform/3/auth/providers/nis/{ProvidersNisId} | 
[**get_providers_summary**](AuthApi.md#get_providers_summary) | **Get** /platform/3/auth/providers/summary | 
[**get_settings_acls**](AuthApi.md#get_settings_acls) | **Get** /platform/3/auth/settings/acls | 
[**get_settings_global**](AuthApi.md#get_settings_global) | **Get** /platform/1/auth/settings/global | 
[**get_settings_krb5_defaults**](AuthApi.md#get_settings_krb5_defaults) | **Get** /platform/1/auth/settings/krb5/defaults | 
[**get_settings_krb5_domain**](AuthApi.md#get_settings_krb5_domain) | **Get** /platform/1/auth/settings/krb5/domains/{SettingsKrb5DomainId} | 
[**get_settings_krb5_realm**](AuthApi.md#get_settings_krb5_realm) | **Get** /platform/1/auth/settings/krb5/realms/{SettingsKrb5RealmId} | 
[**get_settings_mapping**](AuthApi.md#get_settings_mapping) | **Get** /platform/1/auth/settings/mapping | 
[**list_auth_groups**](AuthApi.md#list_auth_groups) | **Get** /platform/1/auth/groups | 
[**list_auth_roles**](AuthApi.md#list_auth_roles) | **Get** /platform/1/auth/roles | 
[**list_auth_users**](AuthApi.md#list_auth_users) | **Get** /platform/1/auth/users | 
[**list_providers_ads**](AuthApi.md#list_providers_ads) | **Get** /platform/3/auth/providers/ads | 
[**list_providers_file**](AuthApi.md#list_providers_file) | **Get** /platform/1/auth/providers/file | 
[**list_providers_krb5**](AuthApi.md#list_providers_krb5) | **Get** /platform/3/auth/providers/krb5 | 
[**list_providers_ldap**](AuthApi.md#list_providers_ldap) | **Get** /platform/4/auth/providers/ldap | 
[**list_providers_nis**](AuthApi.md#list_providers_nis) | **Get** /platform/3/auth/providers/nis | 
[**list_settings_krb5_domains**](AuthApi.md#list_settings_krb5_domains) | **Get** /platform/1/auth/settings/krb5/domains | 
[**list_settings_krb5_realms**](AuthApi.md#list_settings_krb5_realms) | **Get** /platform/1/auth/settings/krb5/realms | 
[**update_auth_group**](AuthApi.md#update_auth_group) | **Put** /platform/1/auth/groups/{AuthGroupId} | 
[**update_auth_log_level**](AuthApi.md#update_auth_log_level) | **Put** /platform/3/auth/log-level | 
[**update_auth_role**](AuthApi.md#update_auth_role) | **Put** /platform/1/auth/roles/{AuthRoleId} | 
[**update_auth_user**](AuthApi.md#update_auth_user) | **Put** /platform/1/auth/users/{AuthUserId} | 
[**update_mapping_import**](AuthApi.md#update_mapping_import) | **Put** /platform/3/auth/mapping/import | 
[**update_mapping_users_rules**](AuthApi.md#update_mapping_users_rules) | **Put** /platform/1/auth/mapping/users/rules | 
[**update_providers_ads_by_id**](AuthApi.md#update_providers_ads_by_id) | **Put** /platform/3/auth/providers/ads/{ProvidersAdsId} | 
[**update_providers_file_by_id**](AuthApi.md#update_providers_file_by_id) | **Put** /platform/1/auth/providers/file/{ProvidersFileId} | 
[**update_providers_krb5_by_id**](AuthApi.md#update_providers_krb5_by_id) | **Put** /platform/3/auth/providers/krb5/{ProvidersKrb5Id} | 
[**update_providers_ldap_by_id**](AuthApi.md#update_providers_ldap_by_id) | **Put** /platform/4/auth/providers/ldap/{ProvidersLdapId} | 
[**update_providers_local_by_id**](AuthApi.md#update_providers_local_by_id) | **Put** /platform/1/auth/providers/local/{ProvidersLocalId} | 
[**update_providers_nis_by_id**](AuthApi.md#update_providers_nis_by_id) | **Put** /platform/3/auth/providers/nis/{ProvidersNisId} | 
[**update_settings_acls**](AuthApi.md#update_settings_acls) | **Put** /platform/3/auth/settings/acls | 
[**update_settings_global**](AuthApi.md#update_settings_global) | **Put** /platform/1/auth/settings/global | 
[**update_settings_krb5_defaults**](AuthApi.md#update_settings_krb5_defaults) | **Put** /platform/1/auth/settings/krb5/defaults | 
[**update_settings_krb5_domain**](AuthApi.md#update_settings_krb5_domain) | **Put** /platform/1/auth/settings/krb5/domains/{SettingsKrb5DomainId} | 
[**update_settings_krb5_realm**](AuthApi.md#update_settings_krb5_realm) | **Put** /platform/1/auth/settings/krb5/realms/{SettingsKrb5RealmId} | 
[**update_settings_mapping**](AuthApi.md#update_settings_mapping) | **Put** /platform/1/auth/settings/mapping | 


# **create_auth_cache_item**
>crate::models::CreateResponse create_auth_cache_item(ctx, auth_cache_item, optional)


Flush the Security Objects Cache.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **auth_cache_item** | [**AuthCacheItem**](AuthCacheItem.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **auth_cache_item** | [**AuthCacheItem**](AuthCacheItem.md)|  | 
 **zone** | **String**| Specifies access zone from which to flush objects. | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_auth_group**
>crate::models::CreateResponse create_auth_group(ctx, auth_group, optional)


Create a new group.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **auth_group** | [**AuthGroupCreateParams**](AuthGroupCreateParams.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **auth_group** | [**AuthGroupCreateParams**](AuthGroupCreateParams.md)|  | 
 **force** | **bool**| Skip validation checks when creating a group. | 
 **zone** | **String**| Optional zone. | 
 **provider** | **String**| Optional provider type. | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_auth_refresh_item**
>crate::models::CreateAuthRefreshItemResponse create_auth_refresh_item(ctx, auth_refresh_item)


Refresh the authentication service configuration.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **auth_refresh_item** | [**Empty**](Empty.md)|  | 

### Return type

[**::models::CreateAuthRefreshItemResponse**](CreateAuthRefreshItemResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_auth_role**
>crate::models::CreateResponse create_auth_role(ctx, auth_role)


Create a new role.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **auth_role** | [**AuthRoleCreateParams**](AuthRoleCreateParams.md)|  | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_auth_user**
>crate::models::CreateResponse create_auth_user(ctx, auth_user, optional)


Create a new user.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **auth_user** | [**AuthUserCreateParams**](AuthUserCreateParams.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **auth_user** | [**AuthUserCreateParams**](AuthUserCreateParams.md)|  | 
 **force** | **bool**| Skip validation checks when creating user. | 
 **zone** | **String**| Optional zone. | 
 **provider** | **String**| Optional provider type. | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_mapping_identity**
>crate::models::Empty create_mapping_identity(ctx, mapping_identity, optional)


Manually set or modify a mapping between two personae.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **mapping_identity** | [**MappingIdentityCreateParams**](MappingIdentityCreateParams.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **mapping_identity** | [**MappingIdentityCreateParams**](MappingIdentityCreateParams.md)|  | 
 **var_2way** | **bool**| Create a bi-directional mapping from source to target and target to source. | 
 **zone** | **String**| Optional zone. | 
 **replace** | **bool**| Replace existing mappings. | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_mapping_identity_0**
>crate::models::MappingIdentities create_mapping_identity_0(ctx, mapping_identity, mapping_identity_id, optional)


Manually set or modify a mapping between two personae.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **mapping_identity** | [**Empty**](Empty.md)|  | 
  **mapping_identity_id** | **String**| Manually set or modify a mapping between two personae. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **mapping_identity** | [**Empty**](Empty.md)|  | 
 **mapping_identity_id** | **String**| Manually set or modify a mapping between two personae. | 
 **_type** | **String**| Desired mapping target to fetch/generate. | 
 **zone** | **String**| Optional zone. | 

### Return type

[**::models::MappingIdentities**](MappingIdentities.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_providers_ads_item**
>crate::models::CreateResponse create_providers_ads_item(ctx, providers_ads_item)


Create a new ADS provider.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **providers_ads_item** | [**ProvidersAdsItem**](ProvidersAdsItem.md)|  | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_providers_file_item**
>crate::models::CreateResponse create_providers_file_item(ctx, providers_file_item)


Create a new file provider.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **providers_file_item** | [**ProvidersFileItem**](ProvidersFileItem.md)|  | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_providers_krb5_item**
>crate::models::CreateResponse create_providers_krb5_item(ctx, providers_krb5_item)


Create a new KRB5 provider.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **providers_krb5_item** | [**ProvidersKrb5Item**](ProvidersKrb5Item.md)|  | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_providers_ldap_item**
>crate::models::CreateResponse create_providers_ldap_item(ctx, providers_ldap_item, optional)


Create a new LDAP provider.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **providers_ldap_item** | [**ProvidersLdapItem**](ProvidersLdapItem.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **providers_ldap_item** | [**ProvidersLdapItem**](ProvidersLdapItem.md)|  | 
 **force** | **bool**| Ignore unresolvable server URIs. | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_providers_nis_item**
>crate::models::CreateResponse create_providers_nis_item(ctx, providers_nis_item)


Create a new NIS provider.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **providers_nis_item** | [**ProvidersNisItem**](ProvidersNisItem.md)|  | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_settings_krb5_domain**
>crate::models::CreateResponse create_settings_krb5_domain(ctx, settings_krb5_domain)


Create a new krb5 domain.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **settings_krb5_domain** | [**SettingsKrb5DomainCreateParams**](SettingsKrb5DomainCreateParams.md)|  | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_settings_krb5_realm**
>crate::models::CreateResponse create_settings_krb5_realm(ctx, settings_krb5_realm)


Create a new krb5 realm.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **settings_krb5_realm** | [**SettingsKrb5RealmCreateParams**](SettingsKrb5RealmCreateParams.md)|  | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_auth_group**
> delete_auth_group(ctx, auth_group_id, optional)


Delete the group.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **auth_group_id** | **String**| Delete the group. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **auth_group_id** | **String**| Delete the group. | 
 **cached** | **bool**| If true, flush the group from the cache. | 
 **zone** | **String**| Filter groups by zone. | 
 **provider** | **String**| Filter groups by provider. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_auth_groups**
> delete_auth_groups(ctx, optional)


Flush the groups cache.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **cached** | **bool**| If true, only flush cached objects. | 
 **zone** | **String**| Filter groups by zone. | 
 **provider** | **String**| Filter groups by provider. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_auth_role**
> delete_auth_role(ctx, auth_role_id)


Delete the role.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **auth_role_id** | **String**| Delete the role. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_auth_user**
> delete_auth_user(ctx, auth_user_id, optional)


Delete the user.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **auth_user_id** | **String**| Delete the user. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **auth_user_id** | **String**| Delete the user. | 
 **cached** | **bool**| If true, flush the user from the cache. | 
 **zone** | **String**| Filter users by zone. | 
 **provider** | **String**| Filter users by provider. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_auth_users**
> delete_auth_users(ctx, optional)


Flush the users cache.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **cached** | **bool**| If true, only flush cached objects. | 
 **zone** | **String**| Filter users by zone. | 
 **provider** | **String**| Filter users by provider. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_mapping_identities**
> delete_mapping_identities(ctx, optional)


Flush the entire idmap cache.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **filter** | **String**| Filter to apply when deleting identity mappings. | 
 **zone** | **String**| Optional zone. | 
 **remove** | **bool**| Delete mapping instead of flush mapping cache. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_mapping_identity**
> delete_mapping_identity(ctx, mapping_identity_id, optional)


Flush the entire idmap cache.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **mapping_identity_id** | **String**| Flush the entire idmap cache. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **mapping_identity_id** | **String**| Flush the entire idmap cache. | 
 **zone** | **String**| Optional zone. | 
 **var_2way** | **bool**| Delete the bi-directional mapping from source to target and target to source. | 
 **target** | **String**| Target identity persona. | 
 **remove** | **bool**| Delete mapping instead of flush mapping from cache. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_providers_ads_by_id**
> delete_providers_ads_by_id(ctx, providers_ads_id)


Delete the ADS provider.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **providers_ads_id** | **String**| Delete the ADS provider. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_providers_file_by_id**
> delete_providers_file_by_id(ctx, providers_file_id)


Delete the file provider.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **providers_file_id** | **String**| Delete the file provider. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_providers_krb5_by_id**
> delete_providers_krb5_by_id(ctx, providers_krb5_id)


Delete the KRB5 provider.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **providers_krb5_id** | **String**| Delete the KRB5 provider. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_providers_ldap_by_id**
> delete_providers_ldap_by_id(ctx, providers_ldap_id)


Delete the LDAP provider.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **providers_ldap_id** | **String**| Delete the LDAP provider. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_providers_local_by_id**
> delete_providers_local_by_id(ctx, providers_local_id)


Delete the local provider.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **providers_local_id** | **String**| Delete the local provider. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_providers_nis_by_id**
> delete_providers_nis_by_id(ctx, providers_nis_id)


Delete the NIS provider.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **providers_nis_id** | **String**| Delete the NIS provider. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_settings_krb5_domain**
> delete_settings_krb5_domain(ctx, settings_krb5_domain_id)


Remove a krb5 domain.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **settings_krb5_domain_id** | **String**| Remove a krb5 domain. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_settings_krb5_realm**
> delete_settings_krb5_realm(ctx, settings_krb5_realm_id)


Remove a realm.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **settings_krb5_realm_id** | **String**| Remove a realm. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_auth_access_user**
>crate::models::AuthAccess get_auth_access_user(ctx, auth_access_user, optional)


Determine user's access rights to a file

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **auth_access_user** | **String**| Determine user&#39;s access rights to a file | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **auth_access_user** | **String**| Determine user&#39;s access rights to a file | 
 **path** | **String**| Path to the file. Must be within /ifs. | 
 **share** | **String**| SMB share name | 
 **zone** | **String**| Access zone the user is in. | 
 **numeric** | **bool**| Show the user&#39;s numeric identifier. | 

### Return type

[**::models::AuthAccess**](AuthAccess.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_auth_group**
>crate::models::AuthGroups get_auth_group(ctx, auth_group_id, optional)


Retrieve the group information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **auth_group_id** | **String**| Retrieve the group information. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **auth_group_id** | **String**| Retrieve the group information. | 
 **cached** | **bool**| If true, only return cached objects. | 
 **resolve_names** | **bool**| Resolve names of personas. | 
 **query_member_of** | **bool**| Enumerate all groups that a group is a member of. | 
 **zone** | **String**| Filter groups by zone. | 
 **provider** | **String**| Filter groups by provider. | 

### Return type

[**::models::AuthGroups**](AuthGroups.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_auth_id**
>crate::models::AuthId get_auth_id(ctx, )


Retrieve the current security token.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::AuthId**](AuthId.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_auth_ldap_template**
>crate::models::AuthLdapTemplates get_auth_ldap_template(ctx, auth_ldap_template_id)


Retrieve the LDAP provider template.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **auth_ldap_template_id** | **String**| Retrieve the LDAP provider template. | 

### Return type

[**::models::AuthLdapTemplates**](AuthLdapTemplates.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_auth_ldap_templates**
>crate::models::AuthLdapTemplatesExtended get_auth_ldap_templates(ctx, )


List all LDAP provider templates.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::AuthLdapTemplatesExtended**](AuthLdapTemplatesExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_auth_log_level**
>crate::models::AuthLogLevel get_auth_log_level(ctx, )


Get the current authentications service and netlogon logging level.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::AuthLogLevel**](AuthLogLevel.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_auth_netgroup**
>crate::models::AuthNetgroups get_auth_netgroup(ctx, auth_netgroup_id, optional)


Retrieve the user information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **auth_netgroup_id** | **String**| Retrieve the user information. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **auth_netgroup_id** | **String**| Retrieve the user information. | 
 **ignore_errors** | **bool**| Ignore netgroup errors. | 
 **recursive** | **bool**| Perform recursive search. | 
 **zone** | **String**| Filter users by zone. | 
 **provider** | **String**| Filter users by provider. | 

### Return type

[**::models::AuthNetgroups**](AuthNetgroups.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_auth_privileges**
>crate::models::AuthPrivileges get_auth_privileges(ctx, )


List all privileges.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::AuthPrivileges**](AuthPrivileges.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_auth_role**
>crate::models::AuthRoles get_auth_role(ctx, auth_role_id, optional)


Retrieve the role information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **auth_role_id** | **String**| Retrieve the role information. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **auth_role_id** | **String**| Retrieve the role information. | 
 **resolve_names** | **bool**| Resolve names of personas. | 

### Return type

[**::models::AuthRoles**](AuthRoles.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_auth_shells**
>crate::models::AuthShells get_auth_shells(ctx, )


List all shells.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::AuthShells**](AuthShells.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_auth_user**
>crate::models::AuthUsers get_auth_user(ctx, auth_user_id, optional)


Retrieve the user information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **auth_user_id** | **String**| Retrieve the user information. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **auth_user_id** | **String**| Retrieve the user information. | 
 **cached** | **bool**| If true, only return cached objects. | 
 **resolve_names** | **bool**| Resolve names of personas. | 
 **query_member_of** | **bool**| Enumerate all users that a group is a member of. | 
 **zone** | **String**| Filter users by zone. | 
 **provider** | **String**| Filter users by provider. | 

### Return type

[**::models::AuthUsers**](AuthUsers.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_auth_wellknown**
>crate::models::AuthWellknowns get_auth_wellknown(ctx, auth_wellknown_id, optional)


Retrieve the wellknown persona.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **auth_wellknown_id** | **String**| Retrieve the wellknown persona. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **auth_wellknown_id** | **String**| Retrieve the wellknown persona. | 
 **scope** | **String**| If specified as \&quot;effective\&quot; or not specified, all fields are returned.  If specified as \&quot;user\&quot;, only fields with non-default values are shown.  If specified as \&quot;default\&quot;, the original values are returned. | 

### Return type

[**::models::AuthWellknowns**](AuthWellknowns.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_auth_wellknowns**
>crate::models::AuthWellknowns get_auth_wellknowns(ctx, )


List all wellknown personas.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::AuthWellknowns**](AuthWellknowns.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_mapping_dump**
>crate::models::MappingDump get_mapping_dump(ctx, optional)


Retrieve all identity mappings (uid, gid, sid, and on-disk) for the supplied source persona.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **nocreate** | **bool**| Idmap should attempt to create missing identity mappings. | 
 **zone** | **String**| Optional zone. | 

### Return type

[**::models::MappingDump**](MappingDump.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_mapping_identity**
>crate::models::MappingIdentities get_mapping_identity(ctx, mapping_identity_id, optional)


Retrieve all identity mappings (uid, gid, sid, and on-disk) for the supplied source persona.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **mapping_identity_id** | **String**| Retrieve all identity mappings (uid, gid, sid, and on-disk) for the supplied source persona. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **mapping_identity_id** | **String**| Retrieve all identity mappings (uid, gid, sid, and on-disk) for the supplied source persona. | 
 **nocreate** | **bool**| Idmap should attempt to create missing identity mappings. | 
 **zone** | **String**| Optional zone. | 

### Return type

[**::models::MappingIdentities**](MappingIdentities.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_mapping_users_lookup**
>crate::models::MappingUsersLookup get_mapping_users_lookup(ctx, optional)


Retrieve the user information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **primary_gid** | **i32**| The user&#39;s primary group ID. | 
 **uid** | **i32**| The user ID. | 
 **zone** | **String**| The zone the user belongs to. | 
 **gid** | [**Vec&lt;i32&gt;**](i32.md)| The IDs of the groups that the user belongs to. | 
 **user** | **String**| The user name. | 
 **kerberos_principal** | **String**| The Kerberos principal name, of the form user@realm. | 

### Return type

[**::models::MappingUsersLookup**](MappingUsersLookup.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_mapping_users_rules**
>crate::models::MappingUsersRules get_mapping_users_rules(ctx, optional)


Retrieve the user mapping rules.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **zone** | **String**| The zone to which the user mapping applies. | 

### Return type

[**::models::MappingUsersRules**](MappingUsersRules.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_providers_ads_by_id**
>crate::models::ProvidersAds get_providers_ads_by_id(ctx, providers_ads_id, optional)


Retrieve the ADS provider.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **providers_ads_id** | **String**| Retrieve the ADS provider. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **providers_ads_id** | **String**| Retrieve the ADS provider. | 
 **scope** | **String**| If specified as \&quot;effective\&quot; or not specified, all fields are returned.  If specified as \&quot;user\&quot;, only fields with non-default values are shown.  If specified as \&quot;default\&quot;, the original values are returned. | 

### Return type

[**::models::ProvidersAds**](ProvidersAds.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_providers_file_by_id**
>crate::models::ProvidersFile get_providers_file_by_id(ctx, providers_file_id, optional)


Retrieve the file provider.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **providers_file_id** | **String**| Retrieve the file provider. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **providers_file_id** | **String**| Retrieve the file provider. | 
 **scope** | **String**| If specified as \&quot;effective\&quot; or not specified, all fields are returned.  If specified as \&quot;user\&quot;, only fields with non-default values are shown.  If specified as \&quot;default\&quot;, the original values are returned. | 

### Return type

[**::models::ProvidersFile**](ProvidersFile.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_providers_krb5_by_id**
>crate::models::ProvidersKrb5 get_providers_krb5_by_id(ctx, providers_krb5_id, optional)


Retrieve the KRB5 provider.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **providers_krb5_id** | **String**| Retrieve the KRB5 provider. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **providers_krb5_id** | **String**| Retrieve the KRB5 provider. | 
 **scope** | **String**| If specified as \&quot;effective\&quot; or not specified, all fields are returned.  If specified as \&quot;user\&quot;, only fields with non-default values are shown.  If specified as \&quot;default\&quot;, the original values are returned. | 

### Return type

[**::models::ProvidersKrb5**](ProvidersKrb5.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_providers_ldap_by_id**
>crate::models::ProvidersLdap get_providers_ldap_by_id(ctx, providers_ldap_id, optional)


Retrieve the LDAP provider.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **providers_ldap_id** | **String**| Retrieve the LDAP provider. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **providers_ldap_id** | **String**| Retrieve the LDAP provider. | 
 **scope** | **String**| If specified as \&quot;effective\&quot; or not specified, all fields are returned.  If specified as \&quot;user\&quot;, only fields with non-default values are shown.  If specified as \&quot;default\&quot;, the original values are returned. | 

### Return type

[**::models::ProvidersLdap**](ProvidersLdap.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_providers_local**
>crate::models::ProvidersLocal get_providers_local(ctx, optional)


List all local providers.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **scope** | **String**| If specified as \&quot;effective\&quot; or not specified, all fields are returned.  If specified as \&quot;user\&quot;, only fields with non-default values are shown.  If specified as \&quot;default\&quot;, the original values are returned. | 

### Return type

[**::models::ProvidersLocal**](ProvidersLocal.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_providers_local_by_id**
>crate::models::ProvidersLocal get_providers_local_by_id(ctx, providers_local_id, optional)


Retrieve the local provider.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **providers_local_id** | **String**| Retrieve the local provider. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **providers_local_id** | **String**| Retrieve the local provider. | 
 **scope** | **String**| If specified as \&quot;effective\&quot; or not specified, all fields are returned.  If specified as \&quot;user\&quot;, only fields with non-default values are shown.  If specified as \&quot;default\&quot;, the original values are returned. | 

### Return type

[**::models::ProvidersLocal**](ProvidersLocal.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_providers_nis_by_id**
>crate::models::ProvidersNis get_providers_nis_by_id(ctx, providers_nis_id, optional)


Retrieve the NIS provider.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **providers_nis_id** | **String**| Retrieve the NIS provider. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **providers_nis_id** | **String**| Retrieve the NIS provider. | 
 **scope** | **String**| If specified as \&quot;effective\&quot; or not specified, all fields are returned.  If specified as \&quot;user\&quot;, only fields with non-default values are shown.  If specified as \&quot;default\&quot;, the original values are returned. | 

### Return type

[**::models::ProvidersNis**](ProvidersNis.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_providers_summary**
>crate::models::ProvidersSummary get_providers_summary(ctx, optional)


Retrieve the summary information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **groupnet** | **String**| Filter providers by groupnet. | 
 **zone** | **String**| Filter providers by zone. | 

### Return type

[**::models::ProvidersSummary**](ProvidersSummary.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_settings_acls**
>crate::models::SettingsAcls get_settings_acls(ctx, optional)


Retrieve the ACL policy settings and preset configurations.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **preset** | **String**| If specified the preset configuration values for all applicable ACL policies are returned. | 

### Return type

[**::models::SettingsAcls**](SettingsAcls.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_settings_global**
>crate::models::SettingsGlobal get_settings_global(ctx, optional)


Retrieve the global settings.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **scope** | **String**| If specified as \&quot;effective\&quot; or not specified, all fields are returned.  If specified as \&quot;user\&quot;, only fields with non-default values are shown.  If specified as \&quot;default\&quot;, the original values are returned. | 
 **zone** | **String**| Zone which contains any per-zone settings. | 

### Return type

[**::models::SettingsGlobal**](SettingsGlobal.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_settings_krb5_defaults**
>crate::models::SettingsKrb5Defaults get_settings_krb5_defaults(ctx, )


Retrieve the krb5 settings.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::SettingsKrb5Defaults**](SettingsKrb5Defaults.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_settings_krb5_domain**
>crate::models::SettingsKrb5Domains get_settings_krb5_domain(ctx, settings_krb5_domain_id)


View the krb5 domain settings.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **settings_krb5_domain_id** | **String**| View the krb5 domain settings. | 

### Return type

[**::models::SettingsKrb5Domains**](SettingsKrb5Domains.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_settings_krb5_realm**
>crate::models::SettingsKrb5Realms get_settings_krb5_realm(ctx, settings_krb5_realm_id)


Retrieve the krb5 settings for realms.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **settings_krb5_realm_id** | **String**| Retrieve the krb5 settings for realms. | 

### Return type

[**::models::SettingsKrb5Realms**](SettingsKrb5Realms.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_settings_mapping**
>crate::models::SettingsMapping get_settings_mapping(ctx, optional)


Retrieve the mapping settings.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **scope** | **String**| If specified as \&quot;effective\&quot; or not specified, all fields are returned.  If specified as \&quot;user\&quot;, only fields with non-default values are shown.  If specified as \&quot;default\&quot;, the original values are returned. | 
 **zone** | **String**| Access zone which contains mapping settings. | 

### Return type

[**::models::SettingsMapping**](SettingsMapping.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_auth_groups**
>crate::models::AuthGroupsExtended list_auth_groups(ctx, optional)


List all groups.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **domain** | **String**| Filter groups by domain. | 
 **zone** | **String**| Filter groups by zone. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 
 **cached** | **bool**| If true, only return cached objects. | 
 **resolve_names** | **bool**| Resolve names of personas. | 
 **filter** | **String**| Filter groups by name prefix. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **provider** | **String**| Filter groups by provider. | 
 **query_member_of** | **bool**| Enumerate all groups that a group is a member of. | 

### Return type

[**::models::AuthGroupsExtended**](AuthGroupsExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_auth_roles**
>crate::models::AuthRolesExtended list_auth_roles(ctx, optional)


List all roles.

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
 **resolve_names** | **bool**| Filter users by zone. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **dir** | **String**| The direction of the sort. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::AuthRolesExtended**](AuthRolesExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_auth_users**
>crate::models::AuthUsersExtended list_auth_users(ctx, optional)


List all users.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **domain** | **String**| Filter users by domain. | 
 **zone** | **String**| Filter users by zone. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 
 **cached** | **bool**| If true, only return cached objects. | 
 **resolve_names** | **bool**| Resolve names of personas. | 
 **filter** | **String**| Filter users by name prefix. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **provider** | **String**| Filter users by provider. | 
 **query_member_of** | **bool**| Enumerate all users that a group is a member of. | 

### Return type

[**::models::AuthUsersExtended**](AuthUsersExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_providers_ads**
>crate::models::ProvidersAdsExtended list_providers_ads(ctx, optional)


List all ADS providers.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **scope** | **String**| If specified as \&quot;effective\&quot; or not specified, all fields are returned.  If specified as \&quot;user\&quot;, only fields with non-default values are shown.  If specified as \&quot;default\&quot;, the original values are returned. | 

### Return type

[**::models::ProvidersAdsExtended**](ProvidersAdsExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_providers_file**
>crate::models::ProvidersFile list_providers_file(ctx, optional)


List all file providers.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **scope** | **String**| If specified as \&quot;effective\&quot; or not specified, all fields are returned.  If specified as \&quot;user\&quot;, only fields with non-default values are shown.  If specified as \&quot;default\&quot;, the original values are returned. | 

### Return type

[**::models::ProvidersFile**](ProvidersFile.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_providers_krb5**
>crate::models::ProvidersKrb5Extended list_providers_krb5(ctx, optional)


List all KRB5 providers.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **scope** | **String**| If specified as \&quot;effective\&quot; or not specified, all fields are returned.  If specified as \&quot;user\&quot;, only fields with non-default values are shown.  If specified as \&quot;default\&quot;, the original values are returned. | 

### Return type

[**::models::ProvidersKrb5Extended**](ProvidersKrb5Extended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_providers_ldap**
>crate::models::ProvidersLdap list_providers_ldap(ctx, optional)


List all LDAP providers.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **scope** | **String**| If specified as \&quot;effective\&quot; or not specified, all fields are returned.  If specified as \&quot;user\&quot;, only fields with non-default values are shown.  If specified as \&quot;default\&quot;, the original values are returned. | 

### Return type

[**::models::ProvidersLdap**](ProvidersLdap.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_providers_nis**
>crate::models::ProvidersNisExtended list_providers_nis(ctx, optional)


List all NIS providers.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **scope** | **String**| If specified as \&quot;effective\&quot; or not specified, all fields are returned.  If specified as \&quot;user\&quot;, only fields with non-default values are shown.  If specified as \&quot;default\&quot;, the original values are returned. | 

### Return type

[**::models::ProvidersNisExtended**](ProvidersNisExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_settings_krb5_domains**
>crate::models::SettingsKrb5Domains list_settings_krb5_domains(ctx, )


Retrieve the krb5 settings for domains.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::SettingsKrb5Domains**](SettingsKrb5Domains.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_settings_krb5_realms**
>crate::models::SettingsKrb5Realms list_settings_krb5_realms(ctx, )


Retrieve the krb5 settings for realms.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::SettingsKrb5Realms**](SettingsKrb5Realms.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_auth_group**
> update_auth_group(ctx, auth_group, auth_group_id, optional)


Modify the group.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **auth_group** | [**AuthGroup**](AuthGroup.md)|  | 
  **auth_group_id** | **String**| Modify the group. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **auth_group** | [**AuthGroup**](AuthGroup.md)|  | 
 **auth_group_id** | **String**| Modify the group. | 
 **force** | **bool**| Changes to the group ID can result in loss of access to the file system. To mitigate this risk of lost access, the force option is required for group ID changes. | 
 **zone** | **String**| Optional zone. | 
 **provider** | **String**| Optional provider type. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_auth_log_level**
> update_auth_log_level(ctx, auth_log_level)


Set the current authentication service and netlogon logging level.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **auth_log_level** | [**AuthLogLevelExtended**](AuthLogLevelExtended.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_auth_role**
> update_auth_role(ctx, auth_role, auth_role_id)


Modify the role.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **auth_role** | [**AuthRole**](AuthRole.md)|  | 
  **auth_role_id** | **String**| Modify the role. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_auth_user**
> update_auth_user(ctx, auth_user, auth_user_id, optional)


Modify the user.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **auth_user** | [**AuthUser**](AuthUser.md)|  | 
  **auth_user_id** | **String**| Modify the user. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **auth_user** | [**AuthUser**](AuthUser.md)|  | 
 **auth_user_id** | **String**| Modify the user. | 
 **force** | **bool**| Changes to the user ID can result in loss of access to the file system. To mitigate this risk of lost access, the force option is required for user ID changes. | 
 **zone** | **String**| Optional zone. | 
 **provider** | **String**| Optional provider type. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_mapping_import**
> update_mapping_import(ctx, mapping_import, optional)


Set or update a list of mappings between two personae.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **mapping_import** | [**MappingImport**](MappingImport.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **mapping_import** | [**MappingImport**](MappingImport.md)|  | 
 **zone** | **String**| Optional zone. | 
 **replace** | **bool**| Specify whether existing mappings should be replaced. The default behavior is to leave existing mappings intact and return an error. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_mapping_users_rules**
> update_mapping_users_rules(ctx, mapping_users_rules, optional)


Modify the user mapping rules.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **mapping_users_rules** | [**MappingUsersRulesExtended**](MappingUsersRulesExtended.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **mapping_users_rules** | [**MappingUsersRulesExtended**](MappingUsersRulesExtended.md)|  | 
 **zone** | **String**| The zone to which the user mapping applies. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_providers_ads_by_id**
> update_providers_ads_by_id(ctx, providers_ads_id_params, providers_ads_id)


Modify the ADS provider.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **providers_ads_id_params** | [**ProvidersAdsIdParams**](ProvidersAdsIdParams.md)|  | 
  **providers_ads_id** | **String**| Modify the ADS provider. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_providers_file_by_id**
> update_providers_file_by_id(ctx, providers_file_id_params, providers_file_id)


Modify the file provider.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **providers_file_id_params** | [**ProvidersFileIdParams**](ProvidersFileIdParams.md)|  | 
  **providers_file_id** | **String**| Modify the file provider. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_providers_krb5_by_id**
> update_providers_krb5_by_id(ctx, providers_krb5_id_params, providers_krb5_id)


Modify the KRB5 provider.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **providers_krb5_id_params** | [**ProvidersKrb5IdParams**](ProvidersKrb5IdParams.md)|  | 
  **providers_krb5_id** | **String**| Modify the KRB5 provider. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_providers_ldap_by_id**
> update_providers_ldap_by_id(ctx, providers_ldap_id_params, providers_ldap_id, optional)


Modify the LDAP provider.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **providers_ldap_id_params** | [**ProvidersLdapIdParams**](ProvidersLdapIdParams.md)|  | 
  **providers_ldap_id** | **String**| Modify the LDAP provider. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **providers_ldap_id_params** | [**ProvidersLdapIdParams**](ProvidersLdapIdParams.md)|  | 
 **providers_ldap_id** | **String**| Modify the LDAP provider. | 
 **force** | **bool**| Ignore unresolvable server URIs. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_providers_local_by_id**
> update_providers_local_by_id(ctx, providers_local_id_params, providers_local_id)


Modify the local provider.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **providers_local_id_params** | [**ProvidersLocalIdParams**](ProvidersLocalIdParams.md)|  | 
  **providers_local_id** | **String**| Modify the local provider. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_providers_nis_by_id**
> update_providers_nis_by_id(ctx, providers_nis_id_params, providers_nis_id)


Modify the NIS provider.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **providers_nis_id_params** | [**ProvidersNisIdParams**](ProvidersNisIdParams.md)|  | 
  **providers_nis_id** | **String**| Modify the NIS provider. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_settings_acls**
> update_settings_acls(ctx, settings_acls)


Modify cluster ACL policy settings.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **settings_acls** | [**SettingsAclsExtended**](SettingsAclsExtended.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_settings_global**
> update_settings_global(ctx, settings_global, optional)


Modify the global settings.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **settings_global** | [**SettingsGlobalGlobalSettings**](SettingsGlobalGlobalSettings.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **settings_global** | [**SettingsGlobalGlobalSettings**](SettingsGlobalGlobalSettings.md)|  | 
 **zone** | **String**| Zone which contains any per-zone settings. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_settings_krb5_defaults**
> update_settings_krb5_defaults(ctx, settings_krb5_defaults)


Modify the krb5 settings.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **settings_krb5_defaults** | [**SettingsKrb5DefaultsKrb5Settings**](SettingsKrb5DefaultsKrb5Settings.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_settings_krb5_domain**
> update_settings_krb5_domain(ctx, settings_krb5_domain, settings_krb5_domain_id)


Modify the krb5 domain settings.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **settings_krb5_domain** | [**SettingsKrb5Domain**](SettingsKrb5Domain.md)|  | 
  **settings_krb5_domain_id** | **String**| Modify the krb5 domain settings. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_settings_krb5_realm**
> update_settings_krb5_realm(ctx, settings_krb5_realm, settings_krb5_realm_id)


Modify the krb5 realm settings.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **settings_krb5_realm** | [**SettingsKrb5Realm**](SettingsKrb5Realm.md)|  | 
  **settings_krb5_realm_id** | **String**| Modify the krb5 realm settings. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_settings_mapping**
> update_settings_mapping(ctx, settings_mapping, optional)


Modify the mapping settings.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **settings_mapping** | [**SettingsMappingMappingSettings**](SettingsMappingMappingSettings.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **settings_mapping** | [**SettingsMappingMappingSettings**](SettingsMappingMappingSettings.md)|  | 
 **zone** | **String**| Access zone which contains mapping settings. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

