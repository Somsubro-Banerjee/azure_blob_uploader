///  # Example
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
///        Ok(value) => println!("Done!"),    // use pattern mathcing to get the output.
///        Err(_) => println!("Some Error Occured!"),
///    }
/// }
/// ```
/// Initialize the `AzureBlobUploader` and call the `upload_file_to_blob_storage()` function
///
///  `file_path` will have the location of the file you want to upload
///  `STORAGE_ACCOUNT` is bascially the name of the storage account
///  `STORAGE_KEY` is the Key to access the storage account
///  `STORAGE_CONTAINER` is the name of the storage container present in the storage account
///  `STORAGE_BLOB` is the name of the blob object.
///  Lastly we are passing the chunk size whihc is of type `usize`.
/// By default the chunk size is set in Mb, so here `10` represents 10Mb chunks.
///
///
/// 
#[doc(html_favicon_url = "https://example.com/favicon.ico")]
#[doc(html_logo_url = "https://example.com/logo.jpg")]

use azure_storage::StorageCredentials;
use azure_storage_blobs::{
    blob::operations::PutBlockListResponse,
    prelude::{BlobBlockType, BlockId, BlockList, ClientBuilder},
};
use std::{fs::File, io::Read};

pub struct AzureBlobUploader;

impl AzureBlobUploader {
    pub async fn upload_file_to_blob_storage(
        file_path: &str,
        storage_account: &str,
        storage_key: &str,
        container_name: &str,
        blob_name: &str,
        chunk_size: usize,
    ) -> Result<PutBlockListResponse, Box<dyn std::error::Error>> {
        let mut file = File::open(&file_path)?;
        let mut total_bytes_uploaded: usize = 0;
        let mut blocks = BlockList::default();

        let storage_credential = StorageCredentials::access_key(storage_account, storage_key);
        let blob_client = ClientBuilder::new(storage_account, storage_credential)
            .blob_client(container_name, blob_name);
        loop {
            let mut buffer = vec![0; chunk_size * 1024 * 1024];

            match file.read(&mut buffer) {
                Ok(n) => {
                    if n == 0 {
                        break;
                    }

                    buffer.truncate(n);

                    let block_id = BlockId::new(format!("{total_bytes_uploaded:016x}"));
                    println!("block id: {block_id:?} {n}");
                    blocks
                        .blocks
                        .push(BlobBlockType::Uncommitted(block_id.clone()));

                    match blob_client.put_block(block_id, buffer).await {
                        Ok(response) => {
                            println!("response: {response:?}");
                            total_bytes_uploaded += n;
                        }
                        Err(error) => {
                            return Err(Box::new(error));
                        }
                    }
                }
                Err(error) => {
                    return Err(Box::new(error));
                }
            }
        }

        match blob_client.put_block_list(blocks).await {
            Ok(put_block_list_response) => {
                let download_url = blob_client.url().unwrap().to_string();
                println!("file is available at {}", download_url);
                Ok(put_block_list_response)
            }
            Err(error) => Err(Box::new(error)),
        }
    }
}
