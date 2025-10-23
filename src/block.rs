use bevy::prelude::*;

#[derive(Component)]
struct BlockState;

#[derive(Component)]
struct BlockModelDefinition(Handle<craftmine_asset::BlockModelDefinition>);

#[derive(Component)]
struct BlockModel;
