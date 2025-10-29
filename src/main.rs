use bevy::prelude::*;

use crate::{
    block::{find_block_model_definitions, load_block_model_definitions},
    world::mesh_chunk,
};

mod block;
mod world;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            craftmine_asset::AssetPlugin,
        ))
        .add_systems(Startup, find_block_model_definitions)
        .add_systems(Update, (load_block_model_definitions, mesh_chunk))
        .run();
}
