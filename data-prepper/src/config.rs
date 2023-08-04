use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub upload: Option<UploadConfig>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UploadConfig {
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: secrecy::SecretString,
    pub bucket: String,
    pub region: String,
}
