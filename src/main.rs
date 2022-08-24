use crate::crypto::sha256_from_bytes;
use crate::network::KeyedClient;
use std::env::args;
use std::fs;
use std::io::{stdout, Write};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

mod crypto;
mod network;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let mut stdout = stdout();
    let args: Vec<String> = args().collect();
    //1. Calculate the hash of a given file (i.e. samplefile.txt)
    let path = Path::new(&args[1]);
    let contents = fs::read(path)?;
    let hash = sha256_from_bytes(&contents)?;
    //2. Perform a hash lookup against metadefender.opswat.com and see if there are
    //previously cached results for the file

    let client = KeyedClient::new()?;
    let data_id = match client.query_hash(&hash).unwrap() {
        //3. If results are found, skip to step 6
        Some(resp_body) => {
            println!("{}",resp_body);
            return Ok(());
        },
        //4. If results are not found, upload the file and receive a "data_id"
        None => {
            let resp_body = client.upload_file(path)?;
            resp_body.data_id
        }
    };
    //5. Repeatedly pull on the "data_id" to retrieve results
    //very simple sleep. A regression or rolling average could be a more sophisticated approach.
    let time_to_next_fetch = Duration::new(5, 0);
    while let resp = client.fetch_analysis(&data_id)? {
        if resp.scan_results.progress_percentage >= 99 {
            //not sure why, but API occasionally hangs at 99 with all engines reporting results.
            //6. Display results in format below (SAMPLE OUTPUT)
            println!("{}", resp);
            return Ok(());
        }
        stdout.flush().unwrap();
        print!(
            "\rstill in progress: {}%",
            resp.scan_results.progress_percentage
        );
        sleep(time_to_next_fetch);
    }
    Ok(())
}
