# CloudSettingsSettingsCloudPolicyDefaults

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**archive_snapshot_files** | **bool** | Specifies if files with snapshots should be archived. | [optional] [default to null]
**cache** | [***::models::CloudSettingsSettingsCloudPolicyDefaultsCache**](CloudSettingsSettingsCloudPolicyDefaultsCache.md) | Specifies default cloudpool cache settings for new filepool policies. | [optional] [default to null]
**compression** | **bool** | Specifies if files should be compressed. | [optional] [default to null]
**data_retention** | **i32** | Specifies the minimum amount of time archived data will be retained in the cloud after deletion. | [optional] [default to null]
**encryption** | **bool** | Specifies if files should be encrypted. | [optional] [default to null]
**full_backup_retention** | **i32** | (Used with NDMP backups only.  Not applicable to SyncIQ.)  The minimum amount of time cloud files will be retained after the creation of a full NDMP backup. | [optional] [default to null]
**incremental_backup_retention** | **i32** | (Used with SyncIQ and NDMP backups.)  The minimum amount of time cloud files will be retained after the creation of a SyncIQ backup or an incremental NDMP backup. | [optional] [default to null]
**writeback_frequency** | **i32** | The minimum amount of time to wait before updating cloud data with local changes. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


