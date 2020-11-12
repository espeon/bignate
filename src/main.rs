#[macro_use] extern crate lazy_static;

mod gocomics;
use gocomics::Comic;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let current_comic = Comic::date("bignate", "11-12-2011").await?;
    dbg!(current_comic);
    Ok(())
}
