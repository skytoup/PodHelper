use std::{collections::HashMap, time::Duration};

use reqwest::Client;
use tokio::sync::Semaphore;

use crate::{common::PodSpec, error::Result, utils};

/// pod spec http client
pub struct PodHTTPClient {
    client: Client,
    semaphore: Semaphore,
}

#[allow(unused_variables)]
impl PodHTTPClient {
    pub fn new(concurrent: usize, connect_sec: u64, timeout_sec: u64) -> PodHTTPClient {
        PodHTTPClient {
            client: Client::builder()
                .connect_timeout(Duration::from_secs(connect_sec))
                .timeout(Duration::from_secs(timeout_sec))
                .gzip(true)
                .build()
                .unwrap(),
            semaphore: Semaphore::new(concurrent),
        }
    }

    pub fn default() -> PodHTTPClient {
        PodHTTPClient::new(5, 10, 25)
    }

    /// fetch some hex pods index
    ///
    /// - url: https://cdn.cocoapods.org/all_pods_versions_2_2_2.txt
    /// - resp(mul lines): AppNetworkManager/1.0.0/1.0.1/1.0.2/1.0.4/1.0.5/1.0.6/1.0.7
    pub async fn fetch_idx_pods(&self, hex: &str) -> Result<String> {
        let sp = self.semaphore.acquire().await;

        let url = format!(
            "https://cdn.cocoapods.org/all_pods_versions_{}.txt",
            utils::str_join(hex, "_"),
        );
        log::debug!("request pod idx {}", hex);
        let resp = self.client.get(&url).send().await?;
        log::debug!("request pod idx {}, resp status {}", hex, resp.status());

        Ok(resp.text().await?)
    }

    /// fetch some hex pods index, and return request hex
    pub async fn fetch_idx_pods_<'a>(&self, hex: &'a str) -> (&'a str, Result<String>) {
        (hex, self.fetch_idx_pods(hex).await)
    }

    /// fetch some version podspec json
    ///
    /// - url: https://cdn.cocoapods.org/Specs/2/2/2/AppNetworkManager/1.0.0/AppNetworkManager.podspec.json
    ///
    pub async fn fetch_ver_podspec_json(
        &self,
        pod: &PodSpec,
        ver: &str,
    ) -> Result<HashMap<String, String>> {
        let sp = self.semaphore.acquire().await;

        let url = format!(
            "https://cdn.cocoapods.org/Specs/{}/{}/{}/{}.podspec.json",
            utils::str_join(&pod.hex, "/"),
            &pod.name,
            ver,
            &pod.name,
        );
        log::debug!("request podspec.json {} {}", pod.name, ver);
        let resp = self.client.get(&url).send().await?;
        log::debug!(
            "request podspec.json {} {}, resp status {}",
            pod.name,
            ver,
            resp.status(),
        );

        Ok(resp.json::<HashMap<String, String>>().await?)
    }
}
