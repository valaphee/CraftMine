use std::collections::HashMap;

use bevy::prelude::*;

#[derive(Component, Default)]
pub struct BlockModel(pub [BlockModelFace; 7]);

#[derive(Default)]
pub struct BlockModelFace {
    pub positions: Vec<Vec3A>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
}

pub fn find_block_model_definitions(asset_server: Res<AssetServer>) {
    std::mem::forget(asset_server.load_folder("minecraft/blockstates"));
}

pub fn load_block_model_definitions(
    block_model_definitions: Res<Assets<craftmine_asset::BlockModelDefinition>>,
    mut block_model_definition_event: MessageReader<
        AssetEvent<craftmine_asset::BlockModelDefinition>,
    >,

    block_models: Res<Assets<craftmine_asset::BlockModel>>,
) {
    if block_model_definition_event.is_empty() {
        return;
    }

    for asset_event in block_model_definition_event.read() {
        let AssetEvent::LoadedWithDependencies { id } = asset_event else {
            continue;
        };

        match block_model_definitions.get(*id).unwrap() {
            craftmine_asset::BlockModelDefinition::Simple(variants) => {
                for (_, models) in variants {
                    let mut parent = block_models.get(&models.first().unwrap().model);

                    let mut textures = HashMap::new();
                    let mut elements = Option::default();
                    while let Some(model) = parent {
                        for texture in &model.textures {
                            textures.entry(texture.0).or_insert(texture.1);
                        }
                        elements.get_or_insert(&model.elements);
                        parent = block_models.get(&model.parent);
                    }

                    bake_block_model(textures, elements.unwrap());
                }
            }
            craftmine_asset::BlockModelDefinition::MultiPart(_multi_part_selectors) => {}
        }
    }
}

pub fn bake_block_model(
    textures: HashMap<&String, &craftmine_asset::Texture>,
    elements: &Vec<craftmine_asset::BlockElement>,
) {
    let mut baked_model = BlockModel::default();
    for element in elements {
        let min = element.from / 16.0;
        let max = element.to / 16.0;

        if let Some(element_face) = &element.faces.down {
            let baked_element_face =
                &mut baked_model.0[element_face.cull.map_or(6, |cull| cull as usize)];
            baked_element_face.positions.extend_from_slice(&[
                Vec3A::new(max.x, min.y, max.z),
                Vec3A::new(min.x, min.y, max.z),
                Vec3A::new(min.x, min.y, min.z),
                Vec3A::new(max.x, min.y, min.z),
            ]);
            baked_element_face.normals.extend_from_slice(&[
                [0.0, -1.0, 0.0],
                [0.0, -1.0, 0.0],
                [0.0, -1.0, 0.0],
                [0.0, -1.0, 0.0],
            ]);
            baked_element_face.uvs.extend_from_slice(&[
                [0.0, 0.0],
                [1.0, 0.0],
                [1.0, 1.0],
                [0.0, 1.0],
            ]);
        }
        if let Some(element_face) = &element.faces.up {
            let baked_element_face =
                &mut baked_model.0[element_face.cull.map_or(6, |cull| cull as usize)];
            baked_element_face.positions.extend_from_slice(&[
                Vec3A::new(max.x, max.y, min.z),
                Vec3A::new(min.x, max.y, min.z),
                Vec3A::new(min.x, max.y, max.z),
                Vec3A::new(max.x, max.y, max.z),
            ]);
            baked_element_face.normals.extend_from_slice(&[
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
            ]);
            baked_element_face.uvs.extend_from_slice(&[
                [1.0, 0.0],
                [0.0, 0.0],
                [0.0, 1.0],
                [1.0, 1.0],
            ]);
        }
        if let Some(element_face) = &element.faces.north {
            let baked_element_face =
                &mut baked_model.0[element_face.cull.map_or(6, |cull| cull as usize)];
            baked_element_face.positions.extend_from_slice(&[
                Vec3A::new(min.x, min.y, max.z),
                Vec3A::new(max.x, min.y, max.z),
                Vec3A::new(max.x, max.y, max.z),
                Vec3A::new(min.x, max.y, max.z),
            ]);
            baked_element_face.normals.extend_from_slice(&[
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
            ]);
            baked_element_face.uvs.extend_from_slice(&[
                [0.0, 0.0],
                [1.0, 0.0],
                [1.0, 1.0],
                [0.0, 1.0],
            ]);
        }
        if let Some(element_face) = &element.faces.south {
            let baked_element_face =
                &mut baked_model.0[element_face.cull.map_or(6, |cull| cull as usize)];
            baked_element_face.positions.extend_from_slice(&[
                Vec3A::new(min.x, max.y, min.z),
                Vec3A::new(max.x, max.y, min.z),
                Vec3A::new(max.x, min.y, min.z),
                Vec3A::new(min.x, min.y, min.z),
            ]);
            baked_element_face.normals.extend_from_slice(&[
                [0.0, 0.0, -1.0],
                [0.0, 0.0, -1.0],
                [0.0, 0.0, -1.0],
                [0.0, 0.0, -1.0],
            ]);
            baked_element_face.uvs.extend_from_slice(&[
                [1.0, 0.0],
                [0.0, 0.0],
                [0.0, 1.0],
                [1.0, 1.0],
            ]);
        }
        if let Some(element_face) = &element.faces.west {
            let baked_element_face =
                &mut baked_model.0[element_face.cull.map_or(6, |cull| cull as usize)];
            baked_element_face.positions.extend_from_slice(&[
                Vec3A::new(max.x, min.y, min.z),
                Vec3A::new(max.x, max.y, min.z),
                Vec3A::new(max.x, max.y, max.z),
                Vec3A::new(max.x, min.y, max.z),
            ]);
            baked_element_face.normals.extend_from_slice(&[
                [1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
            ]);
            baked_element_face.uvs.extend_from_slice(&[
                [0.0, 0.0],
                [1.0, 0.0],
                [1.0, 1.0],
                [0.0, 1.0],
            ]);
        }
        if let Some(element_face) = &element.faces.east {
            let baked_element_face =
                &mut baked_model.0[element_face.cull.map_or(6, |cull| cull as usize)];
            baked_element_face.positions.extend_from_slice(&[
                Vec3A::new(min.x, min.y, max.z),
                Vec3A::new(min.x, max.y, max.z),
                Vec3A::new(min.x, max.y, min.z),
                Vec3A::new(min.x, min.y, min.z),
            ]);
            baked_element_face.normals.extend_from_slice(&[
                [-1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
            ]);
            baked_element_face.uvs.extend_from_slice(&[
                [1.0, 0.0],
                [0.0, 0.0],
                [0.0, 1.0],
                [1.0, 1.0],
            ]);
        }
    }
}
