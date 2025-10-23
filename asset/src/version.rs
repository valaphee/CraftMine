use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Versions {
    pub versions: Vec<VersionsVersion>,
}

#[derive(Serialize, Deserialize)]
pub struct VersionsVersion {
    pub id: String,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Version {
    pub downloads: VersionDownloads,
}

#[derive(Serialize, Deserialize)]
pub struct VersionDownloads {
    pub client: VersionDownload,
}

#[derive(Serialize, Deserialize)]
pub struct VersionDownload {
    pub url: String,
    pub size: u32,
}
