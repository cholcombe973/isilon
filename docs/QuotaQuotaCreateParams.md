# QuotaQuotaCreateParams

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**container** | **bool** | If true, SMB shares using the quota directory see the quota thresholds as share size. | [optional] [default to null]
**enforced** | **bool** | True if the quota provides enforcement, otherwise a accounting quota. | [default to null]
**force** | **bool** | Force creation of quotas on the root of /ifs. | [optional] [default to null]
**include_snapshots** | **bool** | If true, quota governs snapshot data as well as head data. | [default to null]
**path** | **String** | The /ifs path governed. | [default to null]
**persona** | [***::models::AuthAccessAccessItemFileGroup**](AuthAccessAccessItemFileGroup.md) | Specifies properties for a persona, which consists of either a &#39;type&#39; and a &#39;name&#39; or an &#39;ID&#39;. | [optional] [default to null]
**thresholds** | [***::models::QuotaQuotaThresholds**](QuotaQuotaThresholds.md) |  | [optional] [default to null]
**thresholds_include_overhead** | **bool** | If true, thresholds apply to data plus filesystem overhead required to store the data (i.e. &#39;physical&#39; usage). | [default to null]
**_type** | **String** | The type of quota. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


