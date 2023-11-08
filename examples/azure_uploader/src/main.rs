use azure_blob_uploader::azure_blob_uploader::AzureBlobUploader; // Import the Crate
mod secrets;

#[tokio::main]
async fn main() {
    let file_path = "/path/to/your/file"; // Define path to your file
    let uploader = AzureBlobUploader::upload_file_to_blob_storage(
        // Create a Uploader object and call the `azure_blob_uploader()` function.
        file_path,
        secrets::STORAGE_ACCOUNT,
        secrets::STORAGE_KEY,
        secrets::STORAGE_CONTAINER,
        secrets::STORAGE_BLOB,
        10, // The chunk size is in Mb. Here in example the chunk is set to 10Mb.
    )
    .await;
    match uploader {
        Ok(value) => println!("blob url: {:#?}", value), // Use pattern matching to get the output.
        Err(_) => println!("Some Error Occurred!"),
    }
}
