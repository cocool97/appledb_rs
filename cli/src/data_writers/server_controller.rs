use std::collections::BTreeMap;

use anyhow::{Result, bail};
use appledb_common::api_models::{ServerErrorResponse, TaskProgress};
use appledb_common::routes::{ADMIN_ROUTES_PREFIX, PUBLIC_ROUTES_PREFIX};
use appledb_common::{IPSWEntitlements, IPSWFrameworks};
use reqwest::{Client, ClientBuilder, StatusCode};
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::data_writers::data_writer::DataWriter;

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
    pub fn new(server_url: String, insecure: bool) -> Result<Self> {
        let mut client = ClientBuilder::new().gzip(true);
        if insecure {
            client = client
                .danger_accept_invalid_certs(true)
                .danger_accept_invalid_hostnames(true);
        }

        Ok(Self {
            client: client.build()?,
            server_url,
        })
    }

    fn gen_url<S: AsRef<str>>(&self, path: S) -> String {
        format!("{}{}", self.server_url, path.as_ref())
    }

    fn gen_admin_url<S: AsRef<str>>(&self, path: S) -> String {
        self.gen_url(format!("{ADMIN_ROUTES_PREFIX}{}", path.as_ref()))
    }

    fn gen_public_url<S: AsRef<str>>(&self, path: S) -> String {
        self.gen_url(format!("{PUBLIC_ROUTES_PREFIX}{}", path.as_ref()))
    }

    async fn get<T: DeserializeOwned>(&self, url: String) -> Result<T> {
        response_to_result!(self.client.get(&url).send().await?)
    }

    async fn post<D: Serialize, T: DeserializeOwned>(&self, url: String, data: D) -> Result<T> {
        response_to_result!(self.client.post(&url).json(&data).send().await?)
    }

    pub async fn get_running_tasks(&self) -> Result<BTreeMap<String, TaskProgress>> {
        self.get(self.gen_public_url("/tasks/running")).await
    }
}

#[async_trait::async_trait]
impl DataWriter for ServerController {
    async fn post_executable_entitlements(&self, entitlements: IPSWEntitlements) -> Result<()> {
        let task_uuid: String = self
            .post(self.gen_admin_url("/executable/entitlements"), entitlements)
            .await?;

        log::info!("Received entitlements task UUID: {task_uuid}");

        Ok(())
    }

    async fn post_executable_frameworks(&self, frameworks: IPSWFrameworks) -> Result<()> {
        let task_uuid: String = self
            .post(self.gen_admin_url("/executable/frameworks"), frameworks)
            .await?;

        log::info!("Received frameworks task UUID: {task_uuid}");

        Ok(())
    }
}
