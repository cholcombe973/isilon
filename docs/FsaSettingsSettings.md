# FsaSettingsSettings

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default_template** | **String** | Name of question template to use for new FSA jobs. | [optional] [default to null]
**disk_usage_depth** | **i32** | Maximum directory depth used for disk_usage question if not specified in the question. | [optional] [default to null]
**max_age** | **i32** | Maximum age of non-pinned results in seconds. | [optional] [default to null]
**max_count** | **i32** | Maximum number of non-pinned result sets to keep. | [optional] [default to null]
**squash_depth** | **i32** | Squash depth to use for squash binning questions if not specified in the question. | [optional] [default to null]
**top_n_max** | **i32** | Maximum number of items in a Top-N question result if not specified in the question. | [optional] [default to null]
**use_snapshot** | **bool** | If true, use a snapshot for consistency, otherwise analyze head. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


