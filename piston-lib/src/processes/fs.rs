use std::path::{Path, PathBuf};
use zip::ZipArchive;
use async_zip::tokio::read::seek::ZipFileReader;

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
pub fn extract_zip<T: AsRef<Path>>(zip_path: T, output_dir: &PathBuf) {
    println!("Path: {:?}", zip_path.as_ref());
    let file = std::fs::File::open(&zip_path).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = output_dir.join(file.name());

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (&*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.as_path().display());
            std::fs::create_dir_all(&outpath).unwrap();
        } else {
            println!("File {} extracted to \"{}\" ({} bytes)", i, outpath.as_path().display(), file.size());
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = std::fs::File::create(&outpath).unwrap();
            std::io::copy(&mut file, &mut outfile).unwrap();
        }
    }
}