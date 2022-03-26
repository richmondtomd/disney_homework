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

    pub fn download_image(url: String, image_path: String) -> Result<String> {
        match reqwest::blocking::get(format!("{}", url)) {
            Ok(img) => {
                match img.bytes() {
                    Ok(img_bytes) => {
                        match image::load_from_memory(&img_bytes) {
                            Ok(image) => {
                                match image.save(&image_path) {
                                    Ok(_) => {},
                                    Err(err) => {
                                        println!("unsupported fmt");
                                    }
                                }
                            },
                            Err(_) => {
                                println!("unsupported fmt");
                            }
                        }
                    },
                    Err(_) => {
                        println!("unsupported fmt");
                    }
                }
            },
            Err(_) => {
                println!("unsupported fmt");
            },
        }

        Ok(image_path)
    }
}