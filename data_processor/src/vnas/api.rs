use super::api_dtos::ArtccRoot;
use reqwest::{Client, ClientBuilder};
use thiserror::Error;

const BASE_URL: &str = "https://data-api.vnas.vatsim.net/api";

pub struct VnasApi {
    client: Client,
    base_url: &'static str,
}

#[derive(Debug, Error)]
pub enum VnasApiError {
    #[error("Invalid HTTP status code received: {0}")]
    InvalidStatusCode(u16),

    #[error("Error constructing HTTP client")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Failed to serialize/deserialize JSON")]
    FailedJsonParse(#[from] serde_json::Error),
}

impl VnasApi {
    pub fn new() -> Result<Self, VnasApiError> {
        let client = ClientBuilder::new().build()?;
        Ok(Self {
            client,
            base_url: BASE_URL,
        })
    }

    // pub async fn get_artcc_data(&self, artcc_id: &str) -> Result<ArtccRoot, Error> {
    //     let url = format!("{}{}{}", self.base_url, "/artccs/", artcc_id);
    //     let response = self
    //         .client
    //         .get(url)
    //         .send()
    //         .await?
    //         .json::<ArtccRoot>()
    //         .await?;
    //     Ok(response)
    // }

    pub async fn get_all_artccs_data(&self) -> Result<Vec<ArtccRoot>, VnasApiError> {
        let url = format!("{}{}", self.base_url, "/artccs/");
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            Err(VnasApiError::InvalidStatusCode(response.status().as_u16()))
        } else {
            Ok(response.json::<Vec<ArtccRoot>>().await?)
        }
    }
}
