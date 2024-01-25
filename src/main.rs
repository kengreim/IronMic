use crate::vnas_api::VnasApi;
use crate::vnas_models::{AllFacilities, AllPositions};
use reqwest::Error;

mod vnas_api;
mod vnas_models;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // let artccs = vec!["ZAB", "ZMA", "ZOA", "ZHU", "ZBW", "ZDV", "ZID", "ZKC", "ZLA", "ZME", "ZMP", "ZFW", "ZAN", "ZUA", "ZJX", "ZSE", "ZNY", "ZAU", "ZDC", "ZLC", "ZOB", "ZTL", "ZSU", "ZHN"];
    //
    // for artcc in artccs {
    //     let url = format!("https://data-api.vnas.vatsim.net/api/artccs/{artcc}");
    //     println!("Trying {}", url);
    //     let body = get(url)
    //         .await?
    //         .json::<ArtccRoot>()
    //         .await?;
    //
    //     println!("{}", body.facility.name)
    // }

    let x = VnasApi::new().unwrap();
    // let y = x.get_all_artccs_data().await;
    // if let Ok(z) = y {
    //     println!("success")
    // }

    let y = x.get_artcc_data("ZOA").await?;
    for z in y.all_positions() {
        println!("{}", z.callsign)
    }

    Ok(())
}
