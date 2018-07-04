# ProvidersNisNisItem

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**authentication** | **bool** | If true, enables authentication and identity management through the authentication provider. | [optional] [default to null]
**balance_servers** | **bool** | If true, connects the provider to a random server. | [optional] [default to null]
**check_online_interval** | **i32** | Specifies the time in seconds between provider online checks. | [optional] [default to null]
**create_home_directory** | **bool** | Automatically creates the home directory on the first login. | [optional] [default to null]
**enabled** | **bool** | If true, enables the NIS provider. | [optional] [default to null]
**enumerate_groups** | **bool** | If true, allows the provider to enumerate groups. | [optional] [default to null]
**enumerate_users** | **bool** | If true, allows the provider to enumerate users. | [optional] [default to null]
**findable_groups** | **Vec<String>** | Specifies the list of groups that can be resolved. | [optional] [default to null]
**findable_users** | **Vec<String>** | Specifies the list of users that can be resolved. | [optional] [default to null]
**group_domain** | **String** | Specifies the domain for this provider through which groups are qualified. | [optional] [default to null]
**home_directory_template** | **String** | Specifies the path to the home directory template. | [optional] [default to null]
**hostname_lookup** | **bool** | If true, enables host name look ups. | [optional] [default to null]
**id** | **String** | Specifies the NIS provider ID. | [optional] [default to null]
**listable_groups** | **Vec<String>** | Specifies the groups that can be viewed in the provider. | [optional] [default to null]
**listable_users** | **Vec<String>** | Specifies the users that can be viewed in the provider. | [optional] [default to null]
**login_shell** | **String** | Specifies the login shell path. | [optional] [default to null]
**name** | **String** | Specifies the NIS provider name. | [optional] [default to null]
**nis_domain** | **String** | Specifies the NIS domain name. | [optional] [default to null]
**normalize_groups** | **bool** | Normalizes group names to lowercase before look up. | [optional] [default to null]
**normalize_users** | **bool** | Normalizes user names to lowercase before look up. | [optional] [default to null]
**ntlm_support** | **String** | Specifies which NTLM versions to support for users with NTLM-compatible credentials. | [optional] [default to null]
**provider_domain** | **String** | Specifies the domain for the provider. | [optional] [default to null]
**request_timeout** | **i32** | Specifies the request timeout interval in seconds. | [optional] [default to null]
**restrict_findable** | **bool** | If true, checks the provider for filtered lists of findable and unfindable users and groups. | [optional] [default to null]
**restrict_listable** | **bool** | If true, checks the provider for filtered lists of listable and unlistable users and groups. | [optional] [default to null]
**retry_time** | **i32** | Specifies the timeout period in seconds after which a request will be retried. | [optional] [default to null]
**servers** | **Vec<String>** | Adds an NIS server for this provider. | [optional] [default to null]
**status** | **String** | Specifies the status of the provider. | [optional] [default to null]
**system** | **bool** | If true, indicates that this provider instance was created by OneFS and cannot be removed. | [optional] [default to null]
**unfindable_groups** | **Vec<String>** | Specifies groups that cannot be resolved by the provider. | [optional] [default to null]
**unfindable_users** | **Vec<String>** | Specifies users that cannot be resolved by the provider. | [optional] [default to null]
**unlistable_groups** | **Vec<String>** | Specifies a group that cannot be listed by the provider. | [optional] [default to null]
**unlistable_users** | **Vec<String>** | Specifies a user that cannot be listed by the provider. | [optional] [default to null]
**user_domain** | **String** | Specifies the domain for this provider through which users are qualified. | [optional] [default to null]
**ypmatch_using_tcp** | **bool** | If true, specifies TCP for YP Match operations. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


