# DedupeReportExtended

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dedupe_percent** | **String** | The amount of space the directory trees under this job&#39;s paths now take up, compared to what they would take up if not deduplicated (0 ~ 100). | [optional] [default to null]
**elapsed_time** | **i32** | The amount of time in seconds it took to run this job. | [optional] [default to null]
**id** | **i32** | An unique identifier for this report. | [optional] [default to null]
**job_id** | **i32** | The job id this report refers to. | [optional] [default to null]
**job_type** | **String** | The type of dedupe job this report refers to. | [optional] [default to null]
**reports** | [**Vec<::models::DedupeReport>**](DedupeReport.md) | A list of report entries for this dedupe job. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


