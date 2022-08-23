mod crypto;
mod network;

fn main() {
    //1. Calculate the hash of a given file (i.e. samplefile.txt)
    //2. Perform a hash lookup against metadefender.opswat.com and see if there are
    //previously cached results for the file
    //3. If results are found, skip to step 6
    //4. If results are not found, upload the file and receive a "data_id"
    //5. Repeatedly pull on the "data_id" to retrieve results
    //6. Display results in format below (SAMPLE OUTPUT)
    //7. You should also have some basic error handling for common HTTP results
}
