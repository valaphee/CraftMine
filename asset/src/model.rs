use std::collections::HashMap;

use bevy_asset::Asset;
use bevy_reflect::TypePath;
use serde::{Deserialize, Serialize};

use crate::Direction;

#[derive(Serialize, Deserialize, TypePath, Asset, Default)]
pub struct Model {
    #[serde(default)]
    pub parent: Option<String>,
    #[serde(default)]
    pub textures: HashMap<String, String>,
    #[serde(default)]
    pub elements: Vec<ModelElement>,
}

#[derive(Serialize, Deserialize, TypePath)]
pub struct ModelElement {
    pub from: [f32; 3],
    pub to: [f32; 3],
    pub faces: HashMap<Direction, ModelElementFace>,
}

#[derive(Serialize, Deserialize, TypePath)]
pub struct ModelElementFace {
    #[serde(rename = "cullface")]
    pub cull: Option<Direction>,
    pub texture: String,
    pub uv: Option<[f32; 4]>,
}
