use std::collections::HashMap;

use bevy::{
    asset::{Asset, Handle},
    image::Image,
    reflect::TypePath,
};
use serde::{Deserialize, Serialize, de::IntoDeserializer};

use crate::{Direction, ResourceType, resource_location};

#[derive(Serialize, Deserialize, TypePath, Asset)]
pub struct BlockModel {
    #[serde(default, with = "resource_location")]
    pub parent: Handle<BlockModel>,
    #[serde(default)]
    pub textures: HashMap<String, Texture>,
    #[serde(default)]
    pub elements: Vec<BlockElement>,
}

impl ResourceType for BlockModel {
    fn prefix() -> &'static str {
        "models"
    }

    fn extension() -> &'static str {
        "json"
    }
}

pub enum Texture {
    Value(Handle<Image>),
    Reference(String),
}

impl Serialize for Texture {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Texture::Value(handle) => resource_location::serialize(handle, serializer),
            Texture::Reference(reference) => format!("#{reference}").serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Texture {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        if let Some(reference) = value.strip_prefix('#') {
            return Ok(Self::Reference(reference.to_owned()));
        }
        return Ok(Self::Value(resource_location::deserialize(
            value.into_deserializer(),
        )?));
    }
}

#[derive(Serialize, Deserialize)]
pub struct BlockElement {
    pub from: [f32; 3],
    pub to: [f32; 3],
    pub faces: HashMap<Direction, BlockElementFace>,
}

#[derive(Serialize, Deserialize)]
pub struct BlockElementFace {
    #[serde(rename = "cullface")]
    pub cull: Option<Direction>,
    pub texture: Texture,
    pub uv: Option<[f32; 4]>,
}
