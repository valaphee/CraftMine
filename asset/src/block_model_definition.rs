use std::collections::HashMap;

use bevy::{
    asset::{Asset, Handle},
    reflect::TypePath,
};
use serde::{Deserialize, Serialize};
use serde_with::{OneOrMany, serde_as};

use crate::{BlockModel, ResourceType, resource_location};

#[serde_as]
#[derive(Serialize, Deserialize, TypePath, Asset)]
pub enum BlockModelDefinition {
    #[serde(rename = "variants")]
    Simple(#[serde_as(as = "HashMap<_, OneOrMany<_>>")] HashMap<String, Vec<BlockStateModel>>),
    #[serde(rename = "multipart")]
    MultiPart(Vec<MultiPartSelector>),
}

impl ResourceType for BlockModelDefinition {
    fn prefix() -> &'static str {
        "blockstates"
    }

    fn extension() -> &'static str {
        "json"
    }
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct MultiPartSelector {
    #[serde(rename = "apply")]
    #[serde_as(as = "OneOrMany<_>")]
    pub model: Vec<BlockStateModel>,
}

#[derive(Serialize, Deserialize)]
pub struct MultiPartCondition;

#[derive(Serialize, Deserialize)]
pub struct BlockStateModel {
    #[serde(with = "resource_location")]
    pub model: Handle<BlockModel>,
    #[serde(default, rename = "x")]
    pub x_rot: u32,
    #[serde(default, rename = "y")]
    pub y_rot: u32,
    #[serde(default, rename = "uvlock")]
    pub uv_lock: bool,
}
