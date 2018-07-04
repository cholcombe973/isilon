# AntivirusSettingsSettings

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fail_open** | **bool** | Allow access when scanning fails. | [optional] [default to null]
**glob_filters** | **Vec<String>** | Glob patterns for leaf filenames. | [optional] [default to null]
**glob_filters_enabled** | **bool** | Enable glob filters. | [optional] [default to null]
**glob_filters_include** | **bool** | If true, only scan files matching a glob filter. If false, only scan files that don&#39;t match a glob filter. | [optional] [default to null]
**path_prefixes** | **Vec<String>** | Paths to include in realtime scans. | [optional] [default to null]
**quarantine** | **bool** | Try to quarantine files when threats are found. | [optional] [default to null]
**repair** | **bool** | Try to repair files when threats are found. | [optional] [default to null]
**report_expiry** | **i32** | Amount of time in seconds until old reporting data is purged. | [optional] [default to null]
**scan_on_close** | **bool** | Scan files when apps close them. | [optional] [default to null]
**scan_on_open** | **bool** | Scan files on access. | [optional] [default to null]
**scan_size_maximum** | **i32** | Skip scanning files larger than this. | [optional] [default to null]
**service** | **bool** | Whether the antivirus service is enabled. | [optional] [default to null]
**truncate** | **bool** | Try to truncate files when threats are found. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


