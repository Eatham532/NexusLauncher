use std::error::Error;
use std::io::{Read, Write};
use tokio::fs::{copy, File};
use std::path::{PathBuf};
use bytes::{Buf, Bytes};
use reqwest::{Response};
use tokio::fs;
use tokio::io::{AsyncWriteExt, BufWriter};
use crate::processes::network::DownloadResult::{Downloaded, Exists};
use crate::processes::fs::{verify_dir, get_hash};

pub enum DownloadResult {
    Downloaded,
    Exists,
    Error,
}
pub async fn download_from_uri(url: &str, destination: &PathBuf, sha1: Option<&str>, overwrite: bool) -> Result<DownloadResult, Box<dyn Error>> {
    // Check if file exists and overwrite flag is false
    if destination.exists() && !overwrite {
        return Ok(Exists);
    }

    verify_dir(&destination.parent().unwrap().to_path_buf()).await;

    println!("Trying to download from {}", url);
    // Try to download the file up to 4 times
    let mut bytes = read_from_uri(url, sha1).await.unwrap();

    fs::write(&destination, bytes.to_vec()).await?;
    println!("Downloaded successfully to {}", destination.display());
    return Ok(Downloaded);
}

pub async fn read_from_uri(uri:&str, sha1: Option<&str>) -> Result<Bytes, Box<dyn Error>> {
    let client = reqwest::Client::builder().build().unwrap();

    for attempt in 1..=4 {
        let result = client.get(uri).send().await;

        match result {
            Ok(x) => {
                if x.status().is_success() == false {
                    continue;
                }

                println!("Verifying hash");
                let bytes = x.bytes().await;

                if let Ok(bytes) = bytes {
                    if let Some(sha1) = sha1 {
                        if &*get_hash(bytes.clone()).await.unwrap() != sha1 {
                            if attempt <= 3 {
                                continue;
                            } else {
                                eprintln!("Download attempt {} failed: Checksum Error", attempt);
                                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Checksum Error")))
                            }
                        }
                    }
                    return Ok(bytes);
                } else if attempt <= 3 {
                    continue;

                } else if let Err(err) = bytes {
                    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Fetch Error")))
                }
            }
            Err(_) if attempt <= 3 => continue,
            Err(err) => {
                eprintln!("Download attempt {} failed: {}", attempt, err);
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Unknown Error")))
            }
        }

    }


    Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Can't connect to uri {}", uri))))
}

pub async fn unzip_from_uri(url: &str, destination: &PathBuf, sha1: Option<&str>, overwrite: bool) -> Result<(), Box<dyn Error>> {
    let bytes = read_from_uri(url, None).await.unwrap();
    let reader = std::io::Cursor::new(&bytes);

    if let Ok(mut archive) = zip::ZipArchive::new(reader) {
        match archive.extract(destination) {
            Ok(_) => Ok(()),
            Err(err) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Unzip Error: {}", err)))),
        }
    } else {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed extracting native {}", destination.display()))));
    }
}