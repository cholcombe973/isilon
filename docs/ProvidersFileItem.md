# ProvidersFileItem

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**authentication** | **bool** | Enables authentication and identity mapping through the authentication provider. | [optional] [default to null]
**create_home_directory** | **bool** | Automatically creates a home directory on the first login. | [optional] [default to null]
**enabled** | **bool** | Enables the file provider. | [optional] [default to null]
**enumerate_groups** | **bool** | Enables the provider to enumerate groups. | [optional] [default to null]
**enumerate_users** | **bool** | Enables the provider to enumerate users. | [optional] [default to null]
**findable_groups** | **Vec<String>** | Specifies the list of groups that can be resolved. | [optional] [default to null]
**findable_users** | **Vec<String>** | Specifies the list of users that can be resolved. | [optional] [default to null]
**group_domain** | **String** | Specifies the domain for this provider through which domains are qualified. | [optional] [default to null]
**group_file** | **String** | Specifies the location of the file that contains information about the group. | [default to null]
**home_directory_template** | **String** | Specifies the path to the home directory template. | [optional] [default to null]
**listable_groups** | **Vec<String>** | Specifies the groups that can be viewed in the provider. | [optional] [default to null]
**listable_users** | **Vec<String>** | Specifies the users that can be viewed in the provider. | [optional] [default to null]
**login_shell** | **String** | Specifies the login shell path. | [optional] [default to null]
**modifiable_groups** | **Vec<String>** | Specifies the groups that can be modified in the provider. | [optional] [default to null]
**modifiable_users** | **Vec<String>** | Specifies the users that can be modified in the provider. | [optional] [default to null]
**name** | **String** | Specifies the name of the file provider. | [default to null]
**netgroup_file** | **String** | Specifies the path to a netgroups replacement file. | [optional] [default to null]
**normalize_groups** | **bool** | Normalizes group names to lowercase before look up. | [optional] [default to null]
**normalize_users** | **bool** | Normalizes user names to lowercase before look up. | [optional] [default to null]
**ntlm_support** | **String** | Specifies which NTLM versions to support for users with NTLM-compatible credentials. | [optional] [default to null]
**password_file** | **String** | Specifies the location of the file containing information about users. | [default to null]
**provider_domain** | **String** | Specifies the domain for the provider. | [optional] [default to null]
**restrict_findable** | **bool** | If true, checks the provider for filtered lists of findable and unfindable users and groups. | [optional] [default to null]
**restrict_listable** | **bool** | If true, checks the provider for filtered lists of listable and unlistable users and groups. | [optional] [default to null]
**restrict_modifiable** | **bool** | If true, checks the provider for filtered lists of modifiable and unmodifiable users and groups. | [optional] [default to null]
**unfindable_groups** | **Vec<String>** | Specifies groups that cannot be resolved by the provider. | [optional] [default to null]
**unfindable_users** | **Vec<String>** | Specifies users that cannot be resolved by the provider. | [optional] [default to null]
**unlistable_groups** | **Vec<String>** | Specifies a group that cannot be listed by the provider. | [optional] [default to null]
**unlistable_users** | **Vec<String>** | Specifies a user that cannot be listed by the provider. | [optional] [default to null]
**unmodifiable_groups** | **Vec<String>** | Specifies a group that cannot be modified by the provider. | [optional] [default to null]
**unmodifiable_users** | **Vec<String>** | Specifies a user that cannot be modified by the provider. | [optional] [default to null]
**user_domain** | **String** | Specifies the domain for this provider through which users are qualified. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


