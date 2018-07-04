# MappingUsersLookupMappingItemUser

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dn** | **String** |  | [optional] [default to null]
**dns_domain** | **String** |  | [optional] [default to null]
**domain** | **String** |  | [optional] [default to null]
**email** | **String** |  | [optional] [default to null]
**enabled** | **bool** | True, if the authenticated user is enabled. | [default to null]
**expired** | **bool** | True, if the authenticated user has expired. | [default to null]
**expiry** | **i32** |  | [optional] [default to null]
**gecos** | **String** |  | [optional] [default to null]
**generated_gid** | **bool** | True, if the GID was generated. | [optional] [default to null]
**generated_uid** | **bool** | True, if the UID was generated. | [optional] [default to null]
**generated_upn** | **bool** | True, if the UPN was generated. | [optional] [default to null]
**gid** | [***::models::AuthAccessAccessItemFileGroup**](AuthAccessAccessItemFileGroup.md) | Specifies properties for a persona, which consists of either a &#39;type&#39; and a &#39;name&#39; or an &#39;ID&#39;. | [optional] [default to null]
**home_directory** | **String** |  | [optional] [default to null]
**id** | **String** | Specifies the user or group ID. | [default to null]
**locked** | **bool** | If true, indicates that the account is locked. | [default to null]
**max_password_age** | **i32** | Specifies the maximum time in seconds allowed before the password expires. | [optional] [default to null]
**member_of** | [**Vec<::models::AuthAccessAccessItemFileGroup>**](AuthAccessAccessItemFileGroup.md) |  | [optional] [default to null]
**name** | **String** | Specifies a user or group name. | [default to null]
**object_history** | [**Vec<::models::AuthGroupObjectHistoryItem>**](AuthGroupObjectHistoryItem.md) |  | [optional] [default to null]
**on_disk_group_identity** | [***::models::AuthAccessAccessItemFileGroup**](AuthAccessAccessItemFileGroup.md) | Specifies properties for a persona, which consists of either a &#39;type&#39; and a &#39;name&#39; or an &#39;ID&#39;. | [optional] [default to null]
**on_disk_user_identity** | [***::models::AuthAccessAccessItemFileGroup**](AuthAccessAccessItemFileGroup.md) | Specifies properties for a persona, which consists of either a &#39;type&#39; and a &#39;name&#39; or an &#39;ID&#39;. | [optional] [default to null]
**password_expired** | **bool** | If true, the password has expired. | [default to null]
**password_expires** | **bool** | If true, the password is allowed to expire. | [default to null]
**password_expiry** | **i32** |  | [optional] [default to null]
**password_last_set** | **i32** |  | [optional] [default to null]
**primary_group_sid** | [***::models::AuthAccessAccessItemFileGroup**](AuthAccessAccessItemFileGroup.md) | Specifies properties for a persona, which consists of either a &#39;type&#39; and a &#39;name&#39; or an &#39;ID&#39;. | [optional] [default to null]
**prompt_password_change** | **bool** | Prompts the user to change their password at the next login. | [default to null]
**provider** | **String** |  | [optional] [default to null]
**sam_account_name** | **String** |  | [optional] [default to null]
**shell** | **String** |  | [optional] [default to null]
**sid** | [***::models::AuthAccessAccessItemFileGroup**](AuthAccessAccessItemFileGroup.md) | Specifies properties for a persona, which consists of either a &#39;type&#39; and a &#39;name&#39; or an &#39;ID&#39;. | [optional] [default to null]
**_type** | **String** | Specifies the object type. | [default to null]
**uid** | [***::models::AuthAccessAccessItemFileGroup**](AuthAccessAccessItemFileGroup.md) | Specifies properties for a persona, which consists of either a &#39;type&#39; and a &#39;name&#39; or an &#39;ID&#39;. | [optional] [default to null]
**upn** | **String** |  | [optional] [default to null]
**user_can_change_password** | **bool** | Specifies whether the password for the user can be changed. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

