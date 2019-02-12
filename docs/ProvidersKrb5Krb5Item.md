# ProvidersKrb5Krb5Item

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**groupnet** | **String** | Groupnet identifier. | [optional] [default to null]
**id** | **String** | Specifies the Kerberos provider ID. | [optional] [default to null]
**keytab_entries** | [**Vec <crate::models::ProvidersKrb5IdParamsKeytabEntry>**](ProvidersKrb5IdParamsKeytabEntry.md) | Specifies the key information for the Kerberos SPNs. | [optional] [default to null]
**keytab_file** | **String** | Specifies the path to a keytab file to import. | [optional] [default to null]
**manual_keying** | **bool** | If true, keys are managed manually. If false, keys are managed through kadmin. | [optional] [default to null]
**name** | **String** | Specifies the Kerberos provider name. | [optional] [default to null]
**realm** | **String** | Specifies the name of realm. | [optional] [default to null]
**recommended_spns** | **Vec<String>** | Specifies the recommended SPNs. | [optional] [default to null]
**status** | **String** | Specifies the status of the provider. | [optional] [default to null]
**system** | **bool** | If true, indicates that this provider instance was created by OneFS and cannot be removed | [optional] [default to null]
**user** | **String** | Specifies the name of the user that performs kadmin tasks. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


