use bevy::{
    asset::{Asset, Handle, LoadedFolder},
    image::Image,
    reflect::TypePath,
};
use serde::{Deserialize, Serialize};

use crate::ResourceType;

#[derive(Serialize, Deserialize, TypePath, Asset)]
pub struct Atlas {
    sources: Vec<AtlasSource>,
}

#[derive(Serialize, Deserialize)]
pub enum AtlasSource {
    Single { resource: Handle<Image> },
    Directory { source: Handle<LoadedFolder> },
}

impl ResourceType for Atlas {
    fn prefix() -> &'static str {
        "atlases"
    }

    fn extension() -> &'static str {
        "json"
    }
}
