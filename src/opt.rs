use salvo::{writing::Json, Scribe};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub log_path: Option<String>,
    pub cert: String,
    pub key: String,
    pub port: Option<u16>,
    pub metadata: Vec<Metadata>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    name: String,
    hash: String,
}

impl Scribe for Metadata {
    fn render(self, res: &mut salvo::Response) {
        res.render(Json(&self));
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetadataResp(pub Vec<Metadata>);

impl Scribe for MetadataResp {
    fn render(self, res: &mut salvo::Response) {
        res.render(Json(&self));
    }
}
