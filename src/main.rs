use reqwest::{Error, get};
use vnas::ArtccRoot;

mod vnas;

#[tokio::main]
async fn main() -> Result<(), Error>{

    let artccs = vec!["ZAB", "ZMA", "ZOA", "ZHU", "ZBW", "ZDV", "ZID", "ZKC", "ZLA", "ZME", "ZMP", "ZFW", "ZAN", "ZUA", "ZJX", "ZSE", "ZNY", "ZAU", "ZDC", "ZLC", "ZOB", "ZTL", "ZSU", "ZHN"];

    for artcc in artccs {
        let url = format!("https://data-api.vnas.vatsim.net/api/artccs/{artcc}");
        println!("Trying {}", url);
        let body = get(url)
            .await?
            .json::<ArtccRoot>()
            .await?;

        println!("{}", body.facility.name)
    }

    Ok(())
}
