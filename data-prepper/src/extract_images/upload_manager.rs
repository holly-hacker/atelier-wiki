use std::{collections::HashMap, path::Path};

use anyhow::Context;
use s3::{creds::Credentials, Bucket, Region};
use secrecy::ExposeSecret;
use tracing::{debug, info, trace};

use crate::config::UploadConfig;

/// The prefix for all items in the bucket
const PATH_PREFIX: &str = "game-data/";

pub struct UploadManager {
    bucket: Option<Bucket>,
    existing_objects: HashMap<String, [u8; 16]>,
}

impl UploadManager {
    pub fn new() -> Self {
        Self {
            bucket: None,
            existing_objects: Default::default(),
        }
    }

    pub fn load_object_storage(&mut self, config: UploadConfig) -> anyhow::Result<()> {
        let credentials = Credentials::new(
            Some(&config.access_key),
            Some(config.secret_key.expose_secret()),
            None,
            None,
            None,
        )
        .context("create s3 credentials")?;

        // read objects from bucket
        let region = Region::Custom {
            region: config.region,
            endpoint: config.endpoint,
        };
        let bucket = Bucket::new(&config.bucket, region, credentials).context("create bucket")?;

        let pages = bucket
            .list(PATH_PREFIX.to_string(), None)
            .context("list game assets in bucket")?;
        debug!(
            "Found {} pages with {} items",
            pages.len(),
            pages.iter().map(|page| page.contents.len()).sum::<usize>()
        );
        for page in pages {
            for asset in page.contents {
                // for each asset, store the etag so we can later check if it already exists in object storage
                trace!(?asset);
                let tag = asset
                    .e_tag
                    .map(|tag| tag.trim_matches('\"').to_string())
                    .unwrap_or_else(|| "0".repeat(32));
                let mut tag_bytes = [0u8; 16];
                hex::decode_to_slice(tag, &mut tag_bytes as &mut [u8])
                    .context("decode tag to bytes")?;
                self.existing_objects
                    .insert(asset.key.to_string(), tag_bytes);
            }
        }
        debug!(
            count = self.existing_objects.len(),
            "existing objects fetched"
        );
        self.bucket = Some(bucket);

        Ok(())
    }

    pub fn upload(&self, path: &str, data: &[u8]) -> anyhow::Result<()> {
        let Some(bucket) = &self.bucket else { return Ok(()); };

        let object_path = format!("{}{}", PATH_PREFIX, path);
        trace!(?path, ?object_path, "uploading file");

        // see if the object already exists in the remote, and if the md5 matches if it does
        if let Some(existing_md5) = self.existing_objects.get(&object_path) {
            let md5_hash = md5::compute(data).0;
            trace!(?existing_md5, ?md5_hash, "found existing object");
            if md5_hash == *existing_md5 {
                debug!("skipping upload of {} as it already exists", object_path);
                return Ok(());
            } else {
                debug!("md5 mismatch for {}, uploading new copy", object_path);
            }
        }

        info!("Uploading object to s3: {}", object_path);
        if let Some(content_type) = Self::get_content_type(&object_path) {
            bucket
                .put_object_with_content_type(object_path, data, content_type)
                .context("put object with content type")?;
        } else {
            bucket.put_object(object_path, data).context("put object")?;
        }

        Ok(())
    }

    fn get_content_type(key: &str) -> Option<&'static str> {
        let extension = Path::new(key).extension()?.to_str()?;
        match extension {
            "png" => Some("image/png"),
            "jpg" | "jpeg" => Some("image/jpeg"),
            "webp" => Some("image/webp"),
            _ => None,
        }
    }
}
