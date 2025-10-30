use bevy::{camera_controller::free_camera::{FreeCamera, FreeCameraPlugin}, color::palettes::css::WHITE, light::light_consts::lux::{AMBIENT_DAYLIGHT, OVERCAST_DAY, RAW_SUNLIGHT}, prelude::*};

use crate::{
    block::{Block, find_block_textures, find_block_model_definitions, load_block_textures, load_block_model_definitions},
    index::Index,
    world::{Chunk, mesh_chunks},
};

mod block;
mod index;
mod world;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            FreeCameraPlugin,
            craftmine_asset_java::AssetPlugin,
        ))
        .init_state::<AppState>()
        .add_systems(OnEnter(AppState::Load), (find_block_textures, ))
        .add_systems(Update, load_block_textures.run_if(in_state(AppState::Load)))
        .add_systems(Update, (find_block_model_definitions, load_block_model_definitions, load_chunks, mesh_chunks).run_if(in_state(AppState::Loaded)))
        .add_systems(Update, draw_axes)
        .add_systems(Startup, setup)
        .run();
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum AppState {
    #[default]
    Load,
    Loaded,
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,) {
    /*commands.spawn((
        DirectionalLight {
            illuminance: AMBIENT_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        Transform {
                    translation: Vec3::new(0.0, 2.0, 0.0),
                    rotation: Quat::from_rotation_x(-PI / 4.),
                    ..default()
                },
    ));*/
    commands.spawn((
        Camera3d::default(),
        Transform::default(),
        FreeCamera::default(),
        AmbientLight {
                color: WHITE.into(),
                brightness: OVERCAST_DAY,
                ..default()
            },
            Msaa::Off
    ));
}

fn draw_axes(mut gizmos: Gizmos) {
    gizmos.axes(Transform::default(), 8.0);
}

fn load_chunks(mut commands: Commands, blocks: Index<Block>, chunks: Query<&Chunk>) {
    if !chunks.is_empty() {
        return;
    }

    let Some(air) = blocks.get(&Block("air".to_owned())) else {
        return;
    };
    let Some(cmdblock) = blocks.get(&Block("command_block[conditional=false,facing=down]".to_owned())) else {
        return;
    };
    let Some(stone) = blocks.get(&Block("wall_torch[facing=south]".to_owned())) else {
        return;
    };
    let Some(acacia_log_x) = blocks.get(&Block("acacia_log[axis=x]".to_owned())) else {
        return;
    };
    let Some(acacia_log_y) = blocks.get(&Block("acacia_log[axis=y]".to_owned())) else {
        return;
    };
    let Some(acacia_log_z) = blocks.get(&Block("acacia_log[axis=z]".to_owned())) else {
        return;
    };
    let Some(repeater) = blocks.get(&Block("repeater[delay=1,facing=east,locked=true,powered=true]".to_owned())) else {
        return;
    };


    let mut chunk_data = [air; 16 * 16 * 16];
    chunk_data[0] = stone;
    chunk_data[1] = stone;
    chunk_data[2] = stone;
    chunk_data[3] = acacia_log_x;
    chunk_data[4] = acacia_log_y;
    chunk_data[5] = acacia_log_z;
    chunk_data[6] = repeater;
    chunk_data[7] = cmdblock;
    commands.spawn(Chunk(chunk_data));
}
