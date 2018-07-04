# NfsSettingsZoneSettings

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**nfsv4_allow_numeric_ids** | **bool** | If true, sends owners and groups as UIDs and GIDs when look up fails or if the &#39;nfsv4_no_name&#39; property is set to 1. | [optional] [default to null]
**nfsv4_domain** | **String** | Specifies the domain or realm through which users and groups are associated. | [optional] [default to null]
**nfsv4_no_domain** | **bool** | If true, sends owners and groups without a domain name. | [optional] [default to null]
**nfsv4_no_domain_uids** | **bool** | If true, sends UIDs and GIDs without a domain name. | [optional] [default to null]
**nfsv4_no_names** | **bool** | If true, sends owners and groups as UIDs and GIDs. | [optional] [default to null]
**nfsv4_replace_domain** | **bool** | If true, replaces the owner or group domain with an NFS domain name. | [optional] [default to null]
**zone** | **String** | Specifies the access zones in which these settings apply. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


