# NamespaceAcl

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**acl** | [**Vec <crate::models::AclObject>**](AclObject.md) | Provides the JSON array of access rights. | [optional] [default to null]
**authoritative** | **String** | If the directory has access rights set, then this field is returned as acl. If the directory has POSIX permissions set, then this field is returned as mode. | [optional] [default to null]
**group** | [***::models::MemberObject**](MemberObject.md) | Provides the JSON object for the group persona of the owner. | [optional] [default to null]
**mode** | **String** | Provides the POSIX mode. | [optional] [default to null]
**owner** | [***::models::MemberObject**](MemberObject.md) | Provides the JSON object for the owner persona. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


