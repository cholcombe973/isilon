# \NamespaceApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**copy_directory**](NamespaceApi.md#copy_directory) | **Put** /namespace/{DirectoryCopyTarget} | 
[**copy_file**](NamespaceApi.md#copy_file) | **Put** /namespace/{FileCopyTarget} | 
[**create_access_point**](NamespaceApi.md#create_access_point) | **Put** /namespace/{AccessPointName} | 
[**create_directory**](NamespaceApi.md#create_directory) | **Put** /namespace/{DirectoryPath} | 
[**create_file**](NamespaceApi.md#create_file) | **Put** /namespace/{FilePath} | 
[**delete_access_point**](NamespaceApi.md#delete_access_point) | **Delete** /namespace/{AccessPointName} | 
[**delete_directory**](NamespaceApi.md#delete_directory) | **Delete** /namespace/{DirectoryPath} | 
[**delete_file**](NamespaceApi.md#delete_file) | **Delete** /namespace/{FilePath} | 
[**get_acl**](NamespaceApi.md#get_acl) | **Get** /namespace/{NamespacePath} | 
[**get_directory_attributes**](NamespaceApi.md#get_directory_attributes) | **Head** /namespace/{DirectoryPath} | 
[**get_directory_contents**](NamespaceApi.md#get_directory_contents) | **Get** /namespace/{DirectoryPath} | 
[**get_directory_metadata**](NamespaceApi.md#get_directory_metadata) | **Get** /namespace/{DirectoryMetadataPath} | 
[**get_file_attributes**](NamespaceApi.md#get_file_attributes) | **Head** /namespace/{FilePath} | 
[**get_file_contents**](NamespaceApi.md#get_file_contents) | **Get** /namespace/{FilePath} | 
[**get_file_metadata**](NamespaceApi.md#get_file_metadata) | **Get** /namespace/{FileMetadataPath} | 
[**get_worm_properties**](NamespaceApi.md#get_worm_properties) | **Get** /namespace/{WormFilePath} | 
[**list_access_points**](NamespaceApi.md#list_access_points) | **Get** /namespace | 
[**move_directory**](NamespaceApi.md#move_directory) | **Post** /namespace/{DirectoryPath} | 
[**move_file**](NamespaceApi.md#move_file) | **Post** /namespace/{FilePath} | 
[**query_directory**](NamespaceApi.md#query_directory) | **Post** /namespace/{QueryPath} | 
[**set_acl**](NamespaceApi.md#set_acl) | **Put** /namespace/{NamespacePath} | 
[**set_directory_metadata**](NamespaceApi.md#set_directory_metadata) | **Put** /namespace/{DirectoryMetadataPath} | 
[**set_file_metadata**](NamespaceApi.md#set_file_metadata) | **Put** /namespace/{FileMetadataPath} | 
[**set_worm_properties**](NamespaceApi.md#set_worm_properties) | **Put** /namespace/{WormFilePath} | 


# **copy_directory**
>crate::models::CopyErrors copy_directory(ctx, directory_copy_target, x_isi_ifs_copy_source, optional)


Recursively copies a directory to a specified destination path. Symbolic links are copied as regular files.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **directory_copy_target** | **String**| Directory copy destination relative to /. | 
  **x_isi_ifs_copy_source** | **String**| Specifies the full path to the source directory. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **directory_copy_target** | **String**| Directory copy destination relative to /. | 
 **x_isi_ifs_copy_source** | **String**| Specifies the full path to the source directory. | 
 **overwrite** | **bool**| Deletes and replaces the existing user attributes and ACLs of the directory with user-specified attributes and ACLS from the header, when set to true. | 
 **merge** | **bool**| Specifies if the contents of a directory should be merged with an existing directory with the same name. | 
 **_continue** | **bool**| Specifies whether to continue the copy operation on remaining objects when there is a conflict or error. | 

### Return type

[**::models::CopyErrors**](CopyErrors.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **copy_file**
>crate::models::CopyErrors copy_file(ctx, file_copy_target, x_isi_ifs_copy_source, optional)


Copies a file to the specified destination path.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **file_copy_target** | **String**| File copy destination relative to /. | 
  **x_isi_ifs_copy_source** | **String**| Specifies the full path to the source file. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **file_copy_target** | **String**| File copy destination relative to /. | 
 **x_isi_ifs_copy_source** | **String**| Specifies the full path to the source file. | 
 **clone** | **bool**| You must set this parameter to true in order to clone a file. | 
 **snapshot** | **String**| Specifies a snapshot name to clone the file from. | 
 **overwrite** | **bool**| Specifies if an existing file should be overwritten by a new file with the same name. | 

### Return type

[**::models::CopyErrors**](CopyErrors.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_access_point**
>crate::models::Empty create_access_point(ctx, access_point_name, access_point)


Creates a namespace access point in the file system. Only root users can create or change namespace access points.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **access_point_name** | **String**| Access point name. | 
  **access_point** | [**AccessPointCreateParams**](AccessPointCreateParams.md)| Access point parameters model. | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_directory**
>crate::models::Empty create_directory(ctx, directory_path, x_isi_ifs_target_type, optional)


Creates a directory with a specified path.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **directory_path** | **String**| Directory path relative to /. | 
  **x_isi_ifs_target_type** | **String**| Specifies the resource type. | [default to container]
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **directory_path** | **String**| Directory path relative to /. | 
 **x_isi_ifs_target_type** | **String**| Specifies the resource type. | [default to container]
 **x_isi_ifs_access_control** | **String**| Specifies a pre-defined ACL value or POSIX mode with a string in octal string format. | [default to 0700]
 **x_isi_ifs_node_pool_name** | **String**| Specifies a pre-defined ACL value or POSIX mode with a string. | 
 **recursive** | **bool**| Specifies the OneFS node pool name. | 
 **overwrite** | **bool**| Deletes and replaces the existing user attributes and ACLs of the directory with user-specified attributes and ACLS from the header, when set to true. | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_file**
>crate::models::Empty create_file(ctx, file_path, x_isi_ifs_target_type, file_contents, optional)


Creates a file object with a given path. Note that file streaming is not supported.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **file_path** | **String**| File path relative to /. | 
  **x_isi_ifs_target_type** | **String**| Specifies the resource type. | [default to object]
  **file_contents** | **String**| The contents of the file object. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **file_path** | **String**| File path relative to /. | 
 **x_isi_ifs_target_type** | **String**| Specifies the resource type. | [default to object]
 **file_contents** | **String**| The contents of the file object. | 
 **x_isi_ifs_access_control** | **String**| Specifies a pre-defined ACL value or POSIX mode with a string in octal string format. | [default to 0600]
 **content_encoding** | **String**| Specifies the content encoding that was applied to the object content, so that decoding can be applied when retrieving the content. | 
 **content_type** | **String**| Specifies a standard MIME-type description of the content format. | [default to binary/octet-stream]
 **overwrite** | **bool**| Deletes and replaces the existing user attributes and ACLs of the directory with user-specified attributes and ACLS from the header, when set to true. | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_access_point**
>crate::models::Empty delete_access_point(ctx, access_point_name)


Deletes a namespace access point. Only root users can delete namespace access points.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **access_point_name** | **String**| Access point name. | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_directory**
>crate::models::Empty delete_directory(ctx, directory_path, optional)


Deletes the directory at the specified path.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **directory_path** | **String**| Directory path relative to /. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **directory_path** | **String**| Directory path relative to /. | 
 **recursive** | **bool**| Deletes directories recursively, when set to true. | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_file**
>crate::models::Empty delete_file(ctx, file_path)


Deletes the specified file.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **file_path** | **String**| File path relative to /. | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_acl**
>crate::models::NamespaceAcl get_acl(ctx, namespace_path, acl, optional)


Retrieves the access control list for a namespace object.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **namespace_path** | **String**| Namespace path relative to /. | 
  **acl** | **bool**| Show access control lists. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **namespace_path** | **String**| Namespace path relative to /. | 
 **acl** | **bool**| Show access control lists. | 
 **nsaccess** | **bool**| Indicates that the operation is on the access point instead of the store path. | 

### Return type

[**::models::NamespaceAcl**](NamespaceAcl.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_directory_attributes**
>crate::models::Empty get_directory_attributes(ctx, directory_path, optional)


Retrieves the attribute information for a specified directory without transferring the contents of the directory.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **directory_path** | **String**| Directory path relative to /. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **directory_path** | **String**| Directory path relative to /. | 
 **if_modified_since** | **String**| Returns only files that were modified since the specified time. If no files were modified since this time, a 304 message is returned. | 
 **if_unmodified_since** | **String**| Returns only files that were not modified since the specified time. If there are no unmodified files since this time, a 412 message is returned to indicate that the precondition failed. | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_directory_contents**
>crate::models::NamespaceObjects get_directory_contents(ctx, directory_path, optional)


Retrieves a list of files and subdirectories from a directory.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **directory_path** | **String**| Directory path relative to /. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **directory_path** | **String**| Directory path relative to /. | 
 **detail** | **String**| Specifies which object attributes are displayed. | 
 **limit** | **i32**| Specifies the maximum number of objects to send to the client. | 
 **resume** | **String**| Specifies a token to return in the JSON result to indicate when there is a next page. | 
 **sort** | **String**| Specifies one or more attributes to sort on the directory entries. | 
 **dir** | **String**| Specifies the sort direction. This value can be either ascending (ASC) or descending (DESC). | 
 **_type** | **String**| Specifies the object type to return, which can be one of the following values: container, object, pipe, character_device, block_device, symbolic_link, socket, or whiteout_file. | 
 **hidden** | **bool**| Specifies if hidden objects are returned. | 

### Return type

[**::models::NamespaceObjects**](NamespaceObjects.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_directory_metadata**
>crate::models::NamespaceMetadataList get_directory_metadata(ctx, directory_metadata_path, metadata)


Retrieves the attribute information for a specified directory with the metadata query argument.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **directory_metadata_path** | **String**| Directory path relative to /. | 
  **metadata** | **bool**| Show directory metadata. | 

### Return type

[**::models::NamespaceMetadataList**](NamespaceMetadataList.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_file_attributes**
>crate::models::Empty get_file_attributes(ctx, file_path, optional)


Retrieves the attribute information for a specified file.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **file_path** | **String**| File path relative to /. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **file_path** | **String**| File path relative to /. | 
 **if_modified_since** | **String**| Returns only files that were modified since the specified time. If no files were modified since this time, a 304 message is returned. | 
 **if_unmodified_since** | **String**| Returns only files that were not modified since the specified time. If there are no unmodified files since this time, a 412 message is returned to indicate that the precondition failed. | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_file_contents**
>crate::models::Empty get_file_contents(ctx, file_path, optional)


Retrieves the contents of a file from a specified path. Note that file streaming is not supported.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **file_path** | **String**| File path relative to /. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **file_path** | **String**| File path relative to /. | 
 **range** | **String**| Returns the specified range bytes of an object.  | 
 **if_modified_since** | **String**| Returns only files that were modified since the specified time. If no files were modified since this time, a 304 message is returned. | 
 **if_unmodified_since** | **String**| Returns only files that were not modified since the specified time. If there are no unmodified files since this time, a 412 message is returned to indicate that the precondition failed. | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_file_metadata**
>crate::models::NamespaceMetadataList get_file_metadata(ctx, file_metadata_path, metadata)


Retrieves the attribute information for a specified file with the metadata query argument.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **file_metadata_path** | **String**| File path relative to /. | 
  **metadata** | **bool**| Show file metadata. | 

### Return type

[**::models::NamespaceMetadataList**](NamespaceMetadataList.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_worm_properties**
>crate::models::WormProperties get_worm_properties(ctx, worm_file_path, worm)


Retrieves the WORM retention date and committed state of the file.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **worm_file_path** | **String**| Write once read many file path relative to /. | 
  **worm** | **bool**| View WORM properties | 

### Return type

[**::models::WormProperties**](WormProperties.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_access_points**
>crate::models::NamespaceAccessPoints list_access_points(ctx, optional)


Retrieves the namespace access points available for the authenticated user.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **versions** | **bool**| Protocol versions that are supported for the current namespace access server. | 

### Return type

[**::models::NamespaceAccessPoints**](NamespaceAccessPoints.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **move_directory**
>crate::models::Empty move_directory(ctx, directory_path, x_isi_ifs_set_location)


Moves a directory from an existing source to a new destination path.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **directory_path** | **String**| Directory path relative to /. | 
  **x_isi_ifs_set_location** | **String**| Specifies the full path for the destination directory. | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **move_file**
>crate::models::Empty move_file(ctx, file_path, x_isi_ifs_set_location)


Moves a file to a destination path that does not yet exist.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **file_path** | **String**| File path relative to /. | 
  **x_isi_ifs_set_location** | **String**| Specifies the full path for the destination file.  | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **query_directory**
>crate::models::NamespaceObjects query_directory(ctx, query_path, query, directory_query, optional)


Query objects by system-defined and user-defined attributes in a directory.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **query_path** | **String**| Directory path relative to /. | 
  **query** | **bool**| Enable directory query. | 
  **directory_query** | [**DirectoryQuery**](DirectoryQuery.md)| Directory query parameters model. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **query_path** | **String**| Directory path relative to /. | 
 **query** | **bool**| Enable directory query. | 
 **directory_query** | [**DirectoryQuery**](DirectoryQuery.md)| Directory query parameters model. | 
 **limit** | **i32**| Specifies the maximum number of objects to send to the client. You can set the value to a negative number to retrieve all objects. | 
 **detail** | **String**| Specifies which object attributes are displayed. If the detail parameter is excluded, only the name of the object is returned. | 
 **max_depth** | **i32**| Specifies the maximum directory level depth to search for objects. | 

### Return type

[**::models::NamespaceObjects**](NamespaceObjects.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **set_acl**
>crate::models::Empty set_acl(ctx, namespace_path, acl, namespace_acl, optional)


Sets the access control list for a namespace.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **namespace_path** | **String**| Namespace path relative to /. | 
  **acl** | **bool**| Update access control lists. | 
  **namespace_acl** | [**NamespaceAcl**](NamespaceAcl.md)| Namespace ACL parameters model. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **namespace_path** | **String**| Namespace path relative to /. | 
 **acl** | **bool**| Update access control lists. | 
 **namespace_acl** | [**NamespaceAcl**](NamespaceAcl.md)| Namespace ACL parameters model. | 
 **nsaccess** | **bool**| Indicates that the operation is on the access point instead of the store path. | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **set_directory_metadata**
>crate::models::Empty set_directory_metadata(ctx, directory_metadata_path, metadata, directory_metadata)


Sets attributes on a specified directory with the metadata query argument.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **directory_metadata_path** | **String**| Directory path relative to /. | 
  **metadata** | **bool**| Set directory metadata. | 
  **directory_metadata** | [**NamespaceMetadata**](NamespaceMetadata.md)| Directory metadata parameters model. | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **set_file_metadata**
>crate::models::Empty set_file_metadata(ctx, file_metadata_path, metadata, file_metadata)


Sets attributes on a specified file with the metadata query argument through the JSON body.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **file_metadata_path** | **String**| File path relative to /. | 
  **metadata** | **bool**| Set file metadata. | 
  **file_metadata** | [**NamespaceMetadata**](NamespaceMetadata.md)| File metadata parameters model. | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **set_worm_properties**
>crate::models::Empty set_worm_properties(ctx, worm_file_path, worm, worm_properties)


Sets the retention period and commits a file in a SmartLock directory.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **worm_file_path** | **String**| Write once read many file path relative to /. | 
  **worm** | **bool**| View WORM properties | 
  **worm_properties** | [**WormCreateParams**](WormCreateParams.md)| WORM parameters model. | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

