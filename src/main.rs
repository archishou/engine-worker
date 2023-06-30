#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use error_chain::error_chain;
use std::path::Path;
use std::fs::{File};
use std::io::Cursor;
use std::io::prelude::*;

error_chain! {
     foreign_links {
         Io(std::io::Error);
         HttpRequest(reqwest::Error);
     }
}

async fn download_zip() -> Result<()> {
    let url = "https://github.com/twbs/bootstrap/archive/v4.0.0.zip";
    let file_name = "./download.zip";
    let response = reqwest::get(url).await?;
    let mut file = std::fs::File::create(file_name)?;
    let mut content =  Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> Result<()> {
    // Some simple CLI args for later
    /*
    let url = if let Some(url) = std::env::args().nth(1) {
        url
    } else {
        /*
        eprintln!("Missing URL argument");
        process::exit(1);
         */
        println!("No CLI URL provided, using default.");
        "https://hyper.rs".into()
    };
     */

    // hard coded local host for testing
    let url = "http://127.0.0.1:65123/worker-ready";

    eprintln!("Fetching {:?}...", url);

    let res = reqwest::get(url).await?.json::<serde_json::Value>().await?;

    println!("{}", res["repo"]);
    download_zip().await.unwrap();;

    Ok(())
}
