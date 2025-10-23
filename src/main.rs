use bevy::prelude::*;

mod block;
mod world;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            craftmine_asset::AssetPlugin,
        ))
        .run();
}
