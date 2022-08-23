use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::StatusCode;
use serde::Deserialize;
use std::env::{var, VarError};
use std::error::Error;
use std::fs::File;
use std::path::Path;

const BASE_URL: &str = "https://api.metadefender.com/v4/";

#[derive(Deserialize, Debug)]
pub struct AnalyzeFileGoodResponse {
    pub data_id: String,
    pub status: String,
    pub in_queue: i32,
    pub queue_priority: String,
}

pub struct KeyedClient {
    client: Client,
    api_key: String,
}

impl KeyedClient {
    pub fn new() -> Result<KeyedClient, VarError> {
        if let Ok(key) = var("OPSWAT_API_KEY") {
            Ok(KeyedClient {
                client: reqwest::blocking::Client::new(),
                api_key: key,
            })
        } else {
            eprintln!(
                "Environment variable not found. Use the environment variable \"OPSWAT_API_KEY\""
            );
            std::process::exit(2);
        }
    }

    pub fn hash_exists(&self, hash: &str) -> Result<bool, reqwest::Error> {
        let resp = self.get_hash_info(hash)?;
        match resp.status() {
            StatusCode::OK => Ok(true),
            StatusCode::NOT_FOUND => Ok(false),
            _ => panic!(), //unreachable, unreachable! macro is unstable.
        }
    }

    fn get_hash_info(&self, hash: &str) -> reqwest::Result<Response> {
        self.client
            .post(BASE_URL.to_owned() + "hash/" + hash)
            .header("hash", &self.api_key)
            .send()
    }
    pub fn upload_file(&self, path: &Path) -> Result<AnalyzeFileGoodResponse, Box<dyn Error>> {
        //a reasonable default for the demo.
        let mut header_map = HeaderMap::new();
        //api key
        header_map.insert("apikey", HeaderValue::from_str(&self.api_key).unwrap());
        //required. Read all files as binary for the purposes of demo.
        header_map.insert("Content-Type", "application/octet-stream".parse().unwrap());

        header_map.insert(
            "filename",
            HeaderValue::from_str(path.to_str().unwrap()).unwrap(),
        );

        let file = File::open(path)?;

        let resp = self
            .client
            .post(BASE_URL.to_owned() + "file/")
            .headers(header_map)
            .body(file)
            .send()?;

        if resp.status() != reqwest::StatusCode::OK {
            eprintln!(
                "unexpected status code returned from POST analyze file: {:?}\n",
                resp
            );
            eprintln!("{:#?}", resp.text().unwrap());
            std::process::exit(1);
        } else {
            //return type is known from the function's return type,
            //performing the necessary deserialization.
            Ok(resp.json()?)
        }
    }
}
