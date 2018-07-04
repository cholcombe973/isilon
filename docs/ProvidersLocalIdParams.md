# ProvidersLocalIdParams

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**authentication** | **bool** | If true, enables authentication and identity management through the authentication provider. | [optional] [default to null]
**create_home_directory** | **bool** | Automatically creates the home directory on the first login. | [optional] [default to null]
**home_directory_template** | **String** | Specifies the path to the home directory template. | [optional] [default to null]
**lockout_duration** | **i32** | Specifies the length of time in seconds that an account will be inaccessible after multiple failed login attempts. | [optional] [default to null]
**lockout_threshold** | **i32** | Specifies the number of failed login attempts necessary before an account is locked. | [optional] [default to null]
**lockout_window** | **i32** | Specifies the duration of time in seconds in which the number of failed attempts set in the &#39;lockout_threshold&#39; parameter must be made before an account is locked. | [optional] [default to null]
**login_shell** | **String** | Specifies the login shell path. | [optional] [default to null]
**machine_name** | **String** | Specifies the domain for this provider through which users and groups are qualified. | [optional] [default to null]
**max_password_age** | **i32** | Specifies the maximum password age in seconds. | [optional] [default to null]
**min_password_age** | **i32** | Specifies the minimum password age in seconds. | [optional] [default to null]
**min_password_length** | **i32** | Specifies the minimum password length. | [optional] [default to null]
**name** | **String** | Specifies the local provider name. | [optional] [default to null]
**password_complexity** | **Vec<String>** | Specifies the conditions required for a password. | [optional] [default to null]
**password_history_length** | **i32** | Specifies the number of previous passwords to store. | [optional] [default to null]
**password_prompt_time** | **i32** | Specifies the time in seconds remaining before a user will be prompted for a password change. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


