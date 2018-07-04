# AuthUser

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email** | **String** | Specifies an email address for the user. | [optional] [default to null]
**enabled** | **bool** | If true, the authenticated user is enabled. | [optional] [default to null]
**expiry** | **i32** | Specifies the Unix Epoch time when the auth user will expire. | [optional] [default to null]
**gecos** | **String** | Specifies the GECOS value, which is usually the full name. | [optional] [default to null]
**home_directory** | **String** | Specifies a home directory for the user. | [optional] [default to null]
**password** | **String** | Changes the password for the user. | [optional] [default to null]
**password_expires** | **bool** | If true, the password should expire. | [optional] [default to null]
**primary_group** | [***::models::AuthAccessAccessItemFileGroup**](AuthAccessAccessItemFileGroup.md) | Specifies properties for a persona, which consists of either a &#39;type&#39; and a &#39;name&#39; or an &#39;ID&#39;. | [optional] [default to null]
**prompt_password_change** | **bool** | If true, prompts the user to change their password at the next login. | [optional] [default to null]
**shell** | **String** | Specifies the shell for the user. | [optional] [default to null]
**sid** | **String** | Specifies a security identifier. | [optional] [default to null]
**uid** | **i32** | Specifies a numeric user identifier. | [optional] [default to null]
**unlock** | **bool** | If true, the user account should be unlocked. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


