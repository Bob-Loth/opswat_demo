use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::StatusCode;
use serde::Deserialize;
use std::collections::HashMap;
use std::env::{var, VarError};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::path::Path;

const BASE_URL: &str = "https://api.metadefender.com/v4/";

//can grab everything
#[derive(Deserialize, Debug)]
pub struct AnalyzeFileResponse {
    pub data_id: String,
    pub status: String,
    pub in_queue: i32,
    pub queue_priority: String,
}

//or just the fields we care about
#[derive(Deserialize, Debug)]
pub struct UploadFileResponse {
    pub data_id: String,
}

#[derive(Deserialize, Debug)]
pub struct FetchAnalysisResponse {
    pub data_id: String,
    pub scan_results: ScanResults,
}
//6. Display results in format below (SAMPLE OUTPUT)
impl Display for FetchAnalysisResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "overall status: {}",
            self.scan_results.scan_all_result_a
        )?;
        for (name, entry) in &self.scan_results.scan_details {
            writeln!(f, "engine: {}", name)?;
            writeln!(f, "threat_found: {}", entry.threat_found)?;
            writeln!(f, "scan_result: {}", entry.scan_result_i)?;
            writeln!(f, "def_time: {}", entry.def_time)?;
        }
        writeln!(f, "{}", self.scan_results.scan_all_result_a)
    }
}

#[derive(Deserialize, Debug)]
pub struct ScanResults {
    pub scan_details: HashMap<String, EngineResponse>,
    pub progress_percentage: i32,
    pub scan_all_result_a: String,
}

#[derive(Deserialize, Debug)]
pub struct EngineResponse {
    threat_found: String,
    scan_result_i: i32,
    def_time: String,
}

pub struct KeyedClient {
    client: Client,
    api_key: String,
}

impl KeyedClient {
    pub fn new() -> Result<KeyedClient, VarError> {
        if let Ok(key) = var("OPSWAT_API_KEY") {
            Ok(KeyedClient {
                client: Client::new(),
                api_key: key,
            })
        } else {
            eprintln!(
                "Environment variable not found. Use the environment variable \"OPSWAT_API_KEY\""
            );
            std::process::exit(2);
        }
    }

    //returns internal reqwest errors, or an option indicating if the hash has an associated data_id
    pub fn query_hash(&self, hash: &str) -> Result<Option<UploadFileResponse>, reqwest::Error> {
        let resp = self.get_hash_info(hash)?;
        match resp.status() {
            StatusCode::OK => Ok(Some(resp.json()?)),
            StatusCode::NOT_FOUND => Ok(None),
            _ => panic!(), //unreachable, unreachable! macro is unstable.
        }
    }

    fn get_hash_info(&self, hash: &str) -> reqwest::Result<Response> {
        self.client
            .post(BASE_URL.to_owned() + "hash/" + hash)
            .header("hash", &self.api_key)
            .send()
    }
    pub fn upload_file(&self, path: &Path) -> Result<AnalyzeFileResponse, Box<dyn Error>> {
        //a reasonable default for the demo.
        let mut header_map = HeaderMap::new();
        header_map.insert("apikey", HeaderValue::from_str(&self.api_key).unwrap());
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

        if resp.status() != StatusCode::OK {
            eprintln!(
                "unexpected status code returned from POST analyze file: {:?}\n",
                resp
            );
            eprintln!("{:#?}", resp.text().unwrap());
            std::process::exit(1);
        } else {
            //return type is known from the function's return type,
            //performing the necessary deserialization.
            let val = resp.json()?;

            Ok(val)
        }
    }

    pub fn fetch_analysis(
        &self,
        data_id: &String,
    ) -> Result<FetchAnalysisResponse, Box<dyn Error>> {
        let resp = self
            .client
            .get(BASE_URL.to_owned() + "file/" + data_id)
            .header("apikey", &self.api_key)
            .header("x-file-metadata", 0) //documentation on acceptable values for this is unclear
            .send()?;

        match resp.status() {
            StatusCode::NOT_FOUND => {
                eprintln!("Issued a blocking call to upload and retrieve a data_id, but data_id was not found after blocking call.");
                std::process::exit(1);
            }
            StatusCode::OK => {
                let text = resp.text()?;
                eprintln!("{:#?}", &text);
                let json = serde_json::from_str(&text)?;
                eprintln!("{:#?}", &json);
                Ok(json)
            }
            _ => panic!(), //should be unreachable, given good implementation.
        }
    }
}
