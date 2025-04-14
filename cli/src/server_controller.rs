use std::collections::HashMap;

use anyhow::{Result, bail};
use appledb_common::api_models::{ServerErrorResponse, TaskProgress};
use appledb_common::db_models::OperatingSystem;
use appledb_common::routes::{AdminRoutes, PublicRoutes};
use appledb_common::{IPSWEntitlements, IPSWFrameworks};
use reqwest::{Client, ClientBuilder, StatusCode};
use serde::Serialize;
use serde::de::DeserializeOwned;

macro_rules! response_to_result {
    ($response:expr) => {{
        let response = $response;
        match response.status() {
            StatusCode::OK => Ok(response.json::<T>().await?),
            _ => {
                let error_response: ServerErrorResponse = response.json().await?;
                bail!("Server error: {}", error_response.reason)
            }
        }
    }};
}

pub struct ServerController {
    client: Client,
    server_url: String,
}

impl ServerController {
    pub fn new(server_url: String) -> Result<Self> {
        let client = ClientBuilder::new().gzip(true).build()?;

        Ok(Self { client, server_url })
    }

    fn gen_url<S: AsRef<str>>(&self, path: S) -> String {
        format!("{}{}", self.server_url, path.as_ref())
    }

    fn gen_admin_url<S: AsRef<str>>(&self, path: S) -> String {
        self.gen_url(format!("{}{}", AdminRoutes::route_prefix(), path.as_ref()))
    }

    fn gen_public_url<S: AsRef<str>>(&self, path: S) -> String {
        self.gen_url(format!("{}{}", PublicRoutes::route_prefix(), path.as_ref()))
    }

    async fn get<T: DeserializeOwned>(&self, url: String) -> Result<T> {
        response_to_result!(self.client.get(&url).send().await?)
    }

    async fn post<D: Serialize, T: DeserializeOwned>(&self, url: String, data: D) -> Result<T> {
        response_to_result!(self.client.post(&url).json(&data).send().await?)
    }

    pub async fn get_operating_systems(&self) -> Result<Vec<OperatingSystem>> {
        self.get(self.gen_public_url(PublicRoutes::GetOperatingSystems.to_string()))
            .await
    }

    pub async fn post_executable_entitlements(
        &self,
        entitlements: IPSWEntitlements,
    ) -> Result<String> {
        return self
            .post(
                self.gen_admin_url(AdminRoutes::PostExecutableEntitlements.to_string()),
                entitlements,
            )
            .await;
    }

    pub async fn post_executable_frameworks(&self, frameworks: IPSWFrameworks) -> Result<String> {
        return self
            .post(
                self.gen_admin_url(AdminRoutes::PostExecutableFrameworks.to_string()),
                frameworks,
            )
            .await;
    }

    pub async fn get_running_tasks(&self) -> Result<HashMap<String, TaskProgress>> {
        return self
            .get(self.gen_public_url(PublicRoutes::GetRunningTasks.to_string()))
            .await;
    }
}
