/*
 * Bitwarden Internal API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method [`organizations_org_id_auth_requests_deny_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrganizationsOrgIdAuthRequestsDenyPostError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`organizations_org_id_auth_requests_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrganizationsOrgIdAuthRequestsGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`organizations_org_id_auth_requests_request_id_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrganizationsOrgIdAuthRequestsRequestIdPostError {
    UnknownValue(serde_json::Value),
}

pub async fn organizations_org_id_auth_requests_deny_post(
    configuration: &configuration::Configuration,
    org_id: uuid::Uuid,
    bulk_deny_admin_auth_request_request_model: Option<
        crate::models::BulkDenyAdminAuthRequestRequestModel,
    >,
) -> Result<(), Error<OrganizationsOrgIdAuthRequestsDenyPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/organizations/{orgId}/auth-requests/deny",
        local_var_configuration.base_path,
        orgId = crate::apis::urlencode(org_id.to_string())
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&bulk_deny_admin_auth_request_request_model);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<OrganizationsOrgIdAuthRequestsDenyPostError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn organizations_org_id_auth_requests_get(
    configuration: &configuration::Configuration,
    org_id: uuid::Uuid,
) -> Result<
    crate::models::PendingOrganizationAuthRequestResponseModelListResponseModel,
    Error<OrganizationsOrgIdAuthRequestsGetError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/organizations/{orgId}/auth-requests",
        local_var_configuration.base_path,
        orgId = crate::apis::urlencode(org_id.to_string())
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<OrganizationsOrgIdAuthRequestsGetError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn organizations_org_id_auth_requests_request_id_post(
    configuration: &configuration::Configuration,
    org_id: uuid::Uuid,
    request_id: uuid::Uuid,
    admin_auth_request_update_request_model: Option<
        crate::models::AdminAuthRequestUpdateRequestModel,
    >,
) -> Result<(), Error<OrganizationsOrgIdAuthRequestsRequestIdPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/organizations/{orgId}/auth-requests/{requestId}",
        local_var_configuration.base_path,
        orgId = crate::apis::urlencode(org_id.to_string()),
        requestId = crate::apis::urlencode(request_id.to_string())
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&admin_auth_request_update_request_model);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<OrganizationsOrgIdAuthRequestsRequestIdPostError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}