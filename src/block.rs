use std::collections::HashMap;

use bevy::{asset::LoadedFolder, prelude::*};

use crate::{AppState, index::Index};

#[derive(Resource)]
pub struct BlockTexturesFolder(Handle<LoadedFolder>);

#[derive(Resource)]
pub struct BlockTextures {
    texture_atlas: TextureAtlasLayout,
    texture_atlas_sources: TextureAtlasSources,

    pub material: Handle<StandardMaterial>
}

#[derive(Component, PartialEq, Eq, Hash, Clone)]
pub struct Block(pub String);

#[derive(Component, Default)]
pub struct BlockModel(pub [BlockModelFace; 7]);

#[derive(Default)]
pub struct BlockModelFace {
    pub positions: Vec<Vec3>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
}

pub fn find_block_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(BlockTexturesFolder(asset_server.load_folder("minecraft/textures/block")));
}

pub fn load_block_textures(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,

    folder: Res<BlockTexturesFolder>,
    folders: Res<Assets<LoadedFolder>>,
    mut folder_events: MessageReader<AssetEvent<LoadedFolder>>,

    mut textures: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for folder_event in folder_events.read() {
        if !folder_event.is_loaded_with_dependencies(&folder.0) {
            continue;
        }

        let mut texture_atlas_builder = TextureAtlasBuilder::default();
        let folder = folders.get(&folder.0).unwrap();
        for handle in &folder.handles {
            let id = handle.id().typed_unchecked::<Image>();
            let Some(texture) = textures.get(id) else {
                continue;
            };
            texture_atlas_builder.add_texture(Some(id), texture);
        }

        let (texture_atlas, texture_atlas_sources, texture) = texture_atlas_builder.build().unwrap();
        commands.insert_resource(BlockTextures {
            texture_atlas,
            texture_atlas_sources,
            material: materials.add(StandardMaterial {
                base_color_texture: Some(textures.add(texture)),
                reflectance: 0.0,
                ..default()
            }),
        });

        next_state.set(AppState::Loaded);
    }
}

pub fn find_block_model_definitions(asset_server: Res<AssetServer>) {
    std::mem::forget(asset_server.load_folder("minecraft/blockstates"));
}

pub fn load_block_model_definitions(
    mut commands: Commands,

    asset_server: Res<AssetServer>,

    blocks: Index<Block>,

    block_model_definitions: Res<Assets<craftmine_asset_java::BlockModelDefinition>>,
    mut block_model_definition_events: MessageReader<
        AssetEvent<craftmine_asset_java::BlockModelDefinition>,
    >,

    block_models: Res<Assets<craftmine_asset_java::BlockModel>>,
    block_textures: Res<BlockTextures>,
) {
    for block_model_definition_event in block_model_definition_events.read() {
        let AssetEvent::LoadedWithDependencies { id } = block_model_definition_event else {
            continue;
        };

        let path = asset_server.get_path(*id).unwrap();
        let name = path.path().file_prefix().unwrap().to_string_lossy();

        match block_model_definitions.get(*id).unwrap() {
            craftmine_asset_java::BlockModelDefinition::Simple(variants) => {
                for (state, models) in variants {
                    let model = models.first().unwrap();
                    let mut parent = block_models.get(&model.model);

                    let mut textures = HashMap::new();
                    let mut elements = None;
                    while let Some(model) = parent {
                        for texture in &model.textures {
                            textures.entry(texture.0).or_insert(texture.1);
                        }
                        if !model.elements.is_empty() {
                            elements.get_or_insert(&model.elements);
                        }
                        parent = block_models.get(&model.parent);
                    }

                    let block = Block(if state.is_empty() {
                        name.to_string()
                    } else {
                        format!("{name}[{state}]")
                    });
                    let block_model = bake_block_model(&block_textures.texture_atlas, &block_textures.texture_atlas_sources, textures, elements.unwrap_or(&Vec::new()));
                    if let Some(entity) = blocks.get(&block) {
                        commands.entity(entity).insert(block_model);
                    } else {
                        commands.spawn((block, block_model));
                    }
                }
            }
            craftmine_asset_java::BlockModelDefinition::MultiPart(_multi_part_selectors) => {}
        }
    }
}

pub fn bake_block_model(
    texture_atlas: &TextureAtlasLayout,
    texture_atlas_sources: &TextureAtlasSources,

    textures: HashMap<&String, &craftmine_asset_java::Texture>,
    elements: &Vec<craftmine_asset_java::BlockElement>,
) -> BlockModel {
    let mut baked_model = BlockModel::default();
    for element in elements {
        let min = element.from / 16.0;
        let max = element.to / 16.0;
        let rot_pivot = element.rotation.as_ref().map_or(Vec3::ZERO, |rotation| rotation.origin / 16.0);
        let rot = element.rotation.as_ref().map_or(Quat::IDENTITY, |rotation| Quat::from_axis_angle(match rotation.axis {
            craftmine_asset_java::Axis::X => Vec3::X,
            craftmine_asset_java::Axis::Y => Vec3::Y,
            craftmine_asset_java::Axis::Z => Vec3::Z,
        }, rotation.angle.to_radians()));

        if let Some(element_face) = &element.faces.down {
            let mut texture = &element_face.texture;
            while let craftmine_asset_java::Texture::Reference(reference) = texture {
                texture = textures.get(reference).unwrap();
            }
            let craftmine_asset_java::Texture::Value(texture) = texture else {
                panic!()
            };
            let uv_rect = texture_atlas_sources.uv_rect(texture_atlas, texture).unwrap();
            let uv = element_face.uv.map_or([min.x, min.z, max.x, max.z], |uv| [uv[0] / 16.0, uv[1] / 16.0, uv[2] / 16.0, uv[3] / 16.0]);
            let uv_rect = Rect::new(
                uv_rect.min.x.lerp(uv_rect.max.x, uv[0]),
                uv_rect.min.y.lerp(uv_rect.max.y, uv[1]),
                uv_rect.min.x.lerp(uv_rect.max.x, uv[2]),
                uv_rect.min.y.lerp(uv_rect.max.y, uv[3]),
            );

            let baked_element_face =
                &mut baked_model.0[element_face.cull.map_or(6, |cull| cull as usize)];
            baked_element_face.positions.extend_from_slice(&[
                Vec3::new(min.x, min.y, min.z),
                Vec3::new(max.x, min.y, min.z),
                Vec3::new(max.x, min.y, max.z),
                Vec3::new(min.x, min.y, max.z),
            ].map(|point| (rot * (point - rot_pivot)) + rot_pivot));
            baked_element_face.normals.extend_from_slice(&[
                [0.0, -1.0, 0.0],
                [0.0, -1.0, 0.0],
                [0.0, -1.0, 0.0],
                [0.0, -1.0, 0.0],
            ]);
            baked_element_face.uvs.extend_from_slice(&[
                [uv_rect.max.x, uv_rect.min.y],
                [uv_rect.max.x, uv_rect.max.y],
                [uv_rect.min.x, uv_rect.max.y],
                [uv_rect.min.x, uv_rect.min.y],
            ]);
        }

        if let Some(element_face) = &element.faces.up {
            let mut texture = &element_face.texture;
            while let craftmine_asset_java::Texture::Reference(reference) = texture {
                texture = textures.get(reference).unwrap();
            }
            let craftmine_asset_java::Texture::Value(texture) = texture else {
                panic!()
            };
            let uv_rect = texture_atlas_sources.uv_rect(texture_atlas, texture).unwrap();
            let uv = element_face.uv.map_or([min.x, min.z, max.x, max.z], |uv| [uv[0] / 16.0, uv[1] / 16.0, uv[2] / 16.0, uv[3] / 16.0]);
            let uv_rect = Rect::new(
                uv_rect.min.x.lerp(uv_rect.max.x, uv[0]),
                uv_rect.min.y.lerp(uv_rect.max.y, uv[1]),
                uv_rect.min.x.lerp(uv_rect.max.x, uv[2]),
                uv_rect.min.y.lerp(uv_rect.max.y, uv[3]),
            );

            let baked_element_face =
                &mut baked_model.0[element_face.cull.map_or(6, |cull| cull as usize)];
            baked_element_face.positions.extend_from_slice(&[
                Vec3::new(min.x, max.y, max.z),
                Vec3::new(max.x, max.y, max.z),
                Vec3::new(max.x, max.y, min.z),
                Vec3::new(min.x, max.y, min.z),
                ].map(|point| (rot * (point - rot_pivot)) + rot_pivot));
            baked_element_face.normals.extend_from_slice(&[
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
            ]);
            baked_element_face.uvs.extend_from_slice(&[
                [uv_rect.min.x, uv_rect.max.y],
                [uv_rect.max.x, uv_rect.max.y],
                [uv_rect.max.x, uv_rect.min.y],
                [uv_rect.min.x, uv_rect.min.y],
            ]);
        }

        if let Some(element_face) = &element.faces.north {
            let mut texture = &element_face.texture;
            while let craftmine_asset_java::Texture::Reference(reference) = texture {
                texture = textures.get(reference).unwrap();
            }
            let craftmine_asset_java::Texture::Value(texture) = texture else {
                panic!()
            };
            let uv_rect = texture_atlas_sources.uv_rect(texture_atlas, texture).unwrap();
            let uv = element_face.uv.map_or([min.x, min.y, max.x, max.y], |uv| [uv[0] / 16.0, uv[1] / 16.0, uv[2] / 16.0, uv[3] / 16.0]);
            let uv_rect = Rect::new(
                uv_rect.min.x.lerp(uv_rect.max.x, uv[0]),
                uv_rect.min.y.lerp(uv_rect.max.y, uv[1]),
                uv_rect.min.x.lerp(uv_rect.max.x, uv[2]),
                uv_rect.min.y.lerp(uv_rect.max.y, uv[3]),
            );

            let baked_element_face =
                &mut baked_model.0[element_face.cull.map_or(6, |cull| cull as usize)];
            baked_element_face.positions.extend_from_slice(&[
                Vec3::new(max.x, min.y, min.z),
                Vec3::new(min.x, min.y, min.z),
                Vec3::new(min.x, max.y, min.z),
                Vec3::new(max.x, max.y, min.z),
                ].map(|point| (rot * (point - rot_pivot)) + rot_pivot));
            baked_element_face.normals.extend_from_slice(&[
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
            ]);
            baked_element_face.uvs.extend_from_slice(&[
                [uv_rect.min.x, uv_rect.max.y],
                [uv_rect.max.x, uv_rect.max.y],
                [uv_rect.max.x, uv_rect.min.y],
                [uv_rect.min.x, uv_rect.min.y],
            ]);
        }

        if let Some(element_face) = &element.faces.south {
            let mut texture = &element_face.texture;
            while let craftmine_asset_java::Texture::Reference(reference) = texture {
                texture = textures.get(reference).unwrap();
            }
            let craftmine_asset_java::Texture::Value(texture) = texture else {
                panic!()
            };
            let uv_rect = texture_atlas_sources.uv_rect(texture_atlas, texture).unwrap();
            let uv = element_face.uv.map_or([min.x, min.y, max.x, max.y], |uv| [uv[0] / 16.0, uv[1] / 16.0, uv[2] / 16.0, uv[3] / 16.0]);
            let uv_rect = Rect::new(
                uv_rect.min.x.lerp(uv_rect.max.x, uv[0]),
                uv_rect.min.y.lerp(uv_rect.max.y, uv[1]),
                uv_rect.min.x.lerp(uv_rect.max.x, uv[2]),
                uv_rect.min.y.lerp(uv_rect.max.y, uv[3]),
            );

            let baked_element_face =
                &mut baked_model.0[element_face.cull.map_or(6, |cull| cull as usize)];
            baked_element_face.positions.extend_from_slice(&[
                Vec3::new(min.x, min.y, max.z),
                Vec3::new(max.x, min.y, max.z),
                Vec3::new(max.x, max.y, max.z),
                Vec3::new(min.x, max.y, max.z),
                ].map(|point| (rot * (point - rot_pivot)) + rot_pivot));
            baked_element_face.normals.extend_from_slice(&[
                [0.0, 0.0, -1.0],
                [0.0, 0.0, -1.0],
                [0.0, 0.0, -1.0],
                [0.0, 0.0, -1.0],
            ]);
            baked_element_face.uvs.extend_from_slice(&[
                [uv_rect.min.x, uv_rect.max.y],
                [uv_rect.max.x, uv_rect.max.y],
                [uv_rect.max.x, uv_rect.min.y],
                [uv_rect.min.x, uv_rect.min.y],
            ]);
        }

        if let Some(element_face) = &element.faces.west {
            let mut texture = &element_face.texture;
            while let craftmine_asset_java::Texture::Reference(reference) = texture {
                texture = textures.get(reference).unwrap();
            }
            let craftmine_asset_java::Texture::Value(texture) = texture else {
                panic!()
            };
            let uv_rect = texture_atlas_sources.uv_rect(texture_atlas, texture).unwrap();
            let uv = element_face.uv.map_or([min.z, min.y, max.z, max.y], |uv| [uv[0] / 16.0, uv[1] / 16.0, uv[2] / 16.0, uv[3] / 16.0]);
            let uv_rect = Rect::new(
                uv_rect.min.x.lerp(uv_rect.max.x, uv[0]),
                uv_rect.min.y.lerp(uv_rect.max.y, uv[1]),
                uv_rect.min.x.lerp(uv_rect.max.x, uv[2]),
                uv_rect.min.y.lerp(uv_rect.max.y, uv[3]),
            );

            let baked_element_face =
                &mut baked_model.0[element_face.cull.map_or(6, |cull| cull as usize)];
            baked_element_face.positions.extend_from_slice(&[
                Vec3::new(min.x, min.y, min.z),
                Vec3::new(min.x, min.y, max.z),
                Vec3::new(min.x, max.y, max.z),
                Vec3::new(min.x, max.y, min.z),
                ].map(|point| (rot * (point - rot_pivot)) + rot_pivot));
            baked_element_face.normals.extend_from_slice(&[
                [1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
            ]);
            baked_element_face.uvs.extend_from_slice(&[
                [uv_rect.max.x, uv_rect.max.y],
                [uv_rect.min.x, uv_rect.max.y],
                [uv_rect.min.x, uv_rect.min.y],
                [uv_rect.max.x, uv_rect.min.y],
            ]);
        }

        if let Some(element_face) = &element.faces.east {
            let mut texture = &element_face.texture;
            while let craftmine_asset_java::Texture::Reference(reference) = texture {
                texture = textures.get(reference).unwrap();
            }
            let craftmine_asset_java::Texture::Value(texture) = texture else {
                panic!()
            };
            let uv_rect = texture_atlas_sources.uv_rect(texture_atlas, texture).unwrap();
            let uv = element_face.uv.map_or([min.z, min.y, max.z, max.y], |uv| [uv[0] / 16.0, uv[1] / 16.0, uv[2] / 16.0, uv[3] / 16.0]);
            let uv_rect = Rect::new(
                uv_rect.min.x.lerp(uv_rect.max.x, uv[0]),
                uv_rect.min.y.lerp(uv_rect.max.y, uv[1]),
                uv_rect.min.x.lerp(uv_rect.max.x, uv[2]),
                uv_rect.min.y.lerp(uv_rect.max.y, uv[3]),
            );

            let baked_element_face =
                &mut baked_model.0[element_face.cull.map_or(6, |cull| cull as usize)];
            baked_element_face.positions.extend_from_slice(&[
                Vec3::new(max.x, min.y, max.z),
                Vec3::new(max.x, min.y, min.z),
                Vec3::new(max.x, max.y, min.z),
                Vec3::new(max.x, max.y, max.z),
                ].map(|point| (rot * (point - rot_pivot)) + rot_pivot));
            baked_element_face.normals.extend_from_slice(&[
                [-1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
            ]);
            baked_element_face.uvs.extend_from_slice(&[
                [uv_rect.min.x, uv_rect.max.y],
                [uv_rect.max.x, uv_rect.max.y],
                [uv_rect.max.x, uv_rect.min.y],
                [uv_rect.min.x, uv_rect.min.y],
            ]);
        }
    }
    baked_model
}
