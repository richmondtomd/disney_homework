use serde_json::{Result, Value};
use std::fs::File;
use std::io::Read;
use reqwest::Error;
use serde::{Deserialize, de::DeserializeOwned};

pub mod api {
    use super::*;

    pub fn deserialize_api<T>(url: String) -> T 
    where T: 'static + DeserializeOwned {
        let response = requests::get(url).unwrap();
        let api_json = response.text().unwrap();

        // deserialize into objects
        let response: T = serde_json::from_str(&api_json).unwrap();

        response
    }

    // pub fn get_image(url: String) -> Image {
    //     let url = "https://httpbin.org/range/102400?duration=2";
    // const CHUNK_SIZE: u32 = 10240;
        
    // let client = reqwest::blocking::Client::new();
    // let response = client.head(url).send()?;
    // let length = response
    //     .headers()
    //     .get(CONTENT_LENGTH)
    //     .ok_or("response doesn't include the content length")?;
    // let length = u64::from_str(length.to_str()?).map_err(|_| "invalid Content-Length header")?;
        
    // let mut output_file = File::create("download.bin")?;
        
    // println!("starting download...");
    // for range in PartialRangeIter::new(0, length - 1, CHUNK_SIZE)? {
    //     println!("range {:?}", range);
    //     let mut response = client.get(url).header(RANGE, range).send()?;
        
    //     let status = response.status();
    //     if !(status == StatusCode::OK || status == StatusCode::PARTIAL_CONTENT) {
    //     error_chain::bail!("Unexpected server response: {}", status)
    //     }
    //     std::io::copy(&mut response, &mut output_file)?;
    // }
        
    // let content = response.text()?;
    // std::io::copy(&mut content.as_bytes(), &mut output_file)?;

    // println!("Finished with success!");
    // Ok(())
    // }
}