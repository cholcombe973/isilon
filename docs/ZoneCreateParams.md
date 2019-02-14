# ZoneCreateParams

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alternate_system_provider** | **String** | Specifies an alternate system provider. | [optional] [default to null]
**auth_providers** | **Vec<String>** | Specifies the list of authentication providers available on this access zone. | [optional] [default to null]
**cache_entry_expiry** | **i32** | Specifies amount of time in seconds to cache a user/group. | [optional] [default to null]
**create_path** | **bool** | Determines if a path is created when a path does not exist. | [optional] [default to null]
**force_overlap** | **bool** | Allow for overlapping base path. | [optional] [default to null]
**home_directory_umask** | **i32** | Specifies the permissions set on automatically created user home directories. | [optional] [default to null]
**ifs_restricted** | [**Vec <crate::models::AuthAccessAccessItemFileGroup>**](AuthAccessAccessItemFileGroup.md) | Specifies a list of users and groups that have read and write access to /ifs. | [optional] [default to null]
**map_untrusted** | **String** | Maps untrusted domains to this NetBIOS domain during authentication. | [optional] [default to null]
**name** | **String** | Specifies the access zone name. | [default to null]
**negative_cache_entry_expiry** | **i32** | Specifies number of seconds the negative cache entry is valid. | [optional] [default to null]
**netbios_name** | **String** | Specifies the NetBIOS name. | [optional] [default to null]
**path** | **String** | Specifies the access zone base directory path. | [optional] [default to null]
**skeleton_directory** | **String** | Specifies the skeleton directory that is used for user home directories. | [optional] [default to null]
**system_provider** | **String** | Specifies the system provider for the access zone. | [optional] [default to null]
**user_mapping_rules** | **Vec<String>** | Specifies the current ID mapping rules. | [optional] [default to null]
**groupnet** | **String** | Groupnet identitier | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


