use anyhow::{Error, Result};
use regex::Regex;

//bignate_epoch = jan 7, 1991

#[derive(Debug)]
pub struct Comic{
    image_url:String,
    comic_url:String
}

impl Comic {
    pub async fn date(series:&str, date:&str) -> Result<Comic, Error>{

        //TODO: parse date stuff
        let discrim = match date.contains("-") {
            true => "-",
            false => "/",
        };
        let date = date.split(discrim).collect::<Vec<&str>>();

        let comic = Comic::scrape(series, date[2], date[1], date[0]).await?;
        Ok(comic)
    }
    pub async fn scrape(series: &str, y:&str, m:&str, d:&str) -> Result<Comic, Error> {
        let resp =
            reqwest::get(&format!("https://www.gocomics.com/{}/{}/{}/{}", series, y, m, d)).await?;
        assert!(&resp.status().is_success());

        let resp_clone = resp.url().clone();

        let text = &resp.text().await?;

        lazy_static! {
            static ref RE: Regex = Regex::new(r#"<meta property="og:image" content="https://assets\.amuniversal\.com/([^"]+)"\s*/>"#).unwrap();
        }
        let cap = RE.captures(&text).unwrap().get(1).unwrap().as_str();

        let comic = Comic{
            image_url: format!("https://assets.amuniversal.com/{}", &cap),
            comic_url: resp_clone.into_string()
        };

        Ok(comic)
    }
}
