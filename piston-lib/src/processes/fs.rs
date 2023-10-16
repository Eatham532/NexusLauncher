use std::path::PathBuf;

pub async fn verify_dir(path: &PathBuf) {
    if !path.exists() {
        tokio::fs::create_dir_all(&path).await.unwrap();
    }
}

/// Computes a checksum of the input bytes
pub async fn get_hash(bytes: bytes::Bytes) -> Result<String, ()> {
    let hash: String =
        tokio::task::spawn_blocking(|| sha1::Sha1::from(bytes).hexdigest()).await.unwrap();

    Ok(hash)
}