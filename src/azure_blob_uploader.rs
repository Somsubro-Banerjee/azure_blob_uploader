use azure_storage::StorageCredentials;
use azure_storage_blobs::{prelude::{
    BlobBlockType, BlockId, BlockList, ClientBuilder,
}, blob::operations::PutBlockListResponse};
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
                    blocks.blocks.push(BlobBlockType::Uncommitted(block_id.clone()));
    
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
    

