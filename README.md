### Steps:

1. Calculate the hash of a given file (i.e. samplefile.txt)
2. Perform a hash lookup against metadefender.opswat.com and see if there are
previously cached results for the file
3. If results are found, skip to step 6
4. If results are not found, upload the file and receive a "data_id"
5. Repeatedly pull on the "data_id" to retrieve results
6. Display results in format below (SAMPLE OUTPUT)
7. You should also have some basic error handling for common HTTP results

It is not necessary to account for every idiosyncrocy of the API.
You can show any errors to the standard error and exit the application.

### Docs

Pay particular attention to the following
- https://docs.opswat.com/mdcloud/integrations/api-authentication-mechanisms

- https://docs.opswat.com/mdcloud/metadefender-cloud-api-v4/ref#tag-file-
scanning

- https://docs.opswat.com/mdcloud/metadefender-cloud-api-v4/ref#file-
lookupbydataid

- https://docs.opswat.com/mdcloud/metadefender-cloud-api-v4/ref#hash-lookup

#### Build+Test environment:

We will be testing your project on a clean Ubuntu 20.04 VM or latest Visual
Studio 2022 Windows machine.

