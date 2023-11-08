///  Creates a new `AzureBlobUploader` instance
/// # Example
/// ```
/// use azure_blob_uploader::azure_blob_uploader::AzureBlobUploader; // Import the Crate
/// mod secrets;

/// #[tokio::main]
/// async fn main() {
///    let file_path = "/path/to/your/file";     // Define path to your file
///    let uploader = AzureBlobUploader::upload_file_to_blob_storage(      // Create a Uploader object and call the `azure_blob_uploader()` function.
///        file_path,
///        secrets::STORAGE_ACCOUNT,
///        secrets::STORAGE_KEY,
///        secrets::STORAGE_CONTAINER,
///        secrets::STORAGE_BLOB,
///        10,    // The chunk size is in Mb. Here in example the chunk is set to 10Mb.
///    )
///    .await;
///     match uploader {
///        Ok(value) => println!(""blob url: {:#?}", value),    // use pattern mathcing to get the output.
///        Err(_) => println!("Some Error Occured!"),
///    }
/// }
/// ```
/// Initialize the `AzureBlobUploader` and call the `upload_file_to_blob_storage()` function
///
/// `file_path` will have the location of the file you want to upload.<br>
/// `STORAGE_ACCOUNT` is bascially the name of the storage account.<br>
/// `STORAGE_KEY` is the Key to access the storage account.<br>
/// `STORAGE_CONTAINER` is the name of the storage container present in the storage account.<br>
/// `STORAGE_BLOB` is the name of the blob object. <br>
///
/// Lastly we are passing the chunk size which is of type `usize`.
/// By default the chunk size is set in Mb, so here `10` represents 10Mb chunks. <br>
///
/// After the file transfer is completed it returns the link of the storage blob that you can leverage to use it further.
///
pub mod azure_blob_uploader;
