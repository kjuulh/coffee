# \SettingsApi

All URIs are relative to *http://localhost/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_general_api_settings**](SettingsApi.md#get_general_api_settings) | **Get** /settings/api | Get instance&#39;s global settings for api
[**get_general_attachment_settings**](SettingsApi.md#get_general_attachment_settings) | **Get** /settings/attachment | Get instance&#39;s global settings for Attachment
[**get_general_repository_settings**](SettingsApi.md#get_general_repository_settings) | **Get** /settings/repository | Get instance&#39;s global settings for repositories
[**get_general_ui_settings**](SettingsApi.md#get_general_ui_settings) | **Get** /settings/ui | Get instance&#39;s global settings for ui


# **get_general_api_settings**
> ::models::GeneralApiSettings get_general_api_settings(ctx, ctx, ctx, ctx, ctx, ctx, ctx, )
Get instance's global settings for api

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::GeneralApiSettings**](GeneralAPISettings.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

 - **Content-Type**: application/json, text/plain
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_general_attachment_settings**
> ::models::GeneralAttachmentSettings get_general_attachment_settings(ctx, ctx, ctx, ctx, ctx, ctx, ctx, )
Get instance's global settings for Attachment

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::GeneralAttachmentSettings**](GeneralAttachmentSettings.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

 - **Content-Type**: application/json, text/plain
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_general_repository_settings**
> ::models::GeneralRepoSettings get_general_repository_settings(ctx, ctx, ctx, ctx, ctx, ctx, ctx, )
Get instance's global settings for repositories

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::GeneralRepoSettings**](GeneralRepoSettings.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

 - **Content-Type**: application/json, text/plain
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_general_ui_settings**
> ::models::GeneralUiSettings get_general_ui_settings(ctx, ctx, ctx, ctx, ctx, ctx, ctx, )
Get instance's global settings for ui

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::GeneralUiSettings**](GeneralUISettings.md)

### Authorization

[AccessToken](../README.md#AccessToken), [AuthorizationHeaderToken](../README.md#AuthorizationHeaderToken), [BasicAuth](../README.md#BasicAuth), [SudoHeader](../README.md#SudoHeader), [SudoParam](../README.md#SudoParam), [TOTPHeader](../README.md#TOTPHeader), [Token](../README.md#Token)

### HTTP request headers

 - **Content-Type**: application/json, text/plain
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

