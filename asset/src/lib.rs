use bevy::prelude::*;
use serde::{Deserialize, Serialize};

mod block_model;
pub use block_model::*;

mod block_model_definition;
pub use block_model_definition::*;

mod resource;
pub use resource::*;

mod version;
pub use version::*;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<BlockModel>()
            .init_asset::<BlockModelDefinition>()
            .init_asset_loader::<JsonLoader<BlockModel>>()
            .init_asset_loader::<JsonLoader<BlockModelDefinition>>();
    }
}

#[derive(Serialize, Deserialize, Reflect, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Down,
    Up,
    North,
    South,
    West,
    East,
}
