use super::api_dtos::ArtccRoot;
use reqwest::{Client, ClientBuilder, Error};

const BASE_URL: &str = "https://data-api.vnas.vatsim.net/api";

pub struct VnasApi {
    client: Client,
    base_url: &'static str,
}

impl VnasApi {
    pub fn new() -> Result<Self, Error> {
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

    pub async fn get_all_artccs_data(&self) -> Result<Vec<ArtccRoot>, Error> {
        let url = format!("{}{}", self.base_url, "/artccs/");
        let response = self
            .client
            .get(url)
            .send()
            .await?
            .json::<Vec<ArtccRoot>>()
            .await?;
        Ok(response)
    }
}
