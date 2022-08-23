use crate::crypto::sha256_from_bytes;
use crate::network::KeyedClient;
use std::fs;
use std::path::Path;

mod crypto;
mod network;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    //1. Calculate the hash of a given file (i.e. samplefile.txt)
    let path = Path::new("tests/resources/test.txt");
    let contents = fs::read(path)?;
    let hash = sha256_from_bytes(&contents)?;
    println!("{}", hash);
    //2. Perform a hash lookup against metadefender.opswat.com and see if there are
    //previously cached results for the file

    let client = KeyedClient::new()?;
    if client.hash_exists(&hash)? {
        //print out response
    } else {
        //a valid not found response
        let json_data = client.upload_file(path)?;
        println!("{:#?}", json_data);
    }

    //3. If results are found, skip to step 6
    //4. If results are not found, upload the file and receive a "data_id"
    //5. Repeatedly pull on the "data_id" to retrieve results
    //6. Display results in format below (SAMPLE OUTPUT)
    //7. You should also have some basic error handling for common HTTP results
    Ok(())
}
