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
}