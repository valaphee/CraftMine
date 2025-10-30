use bevy::{asset::RenderAssetUsages, mesh::Indices, prelude::*};

use crate::block::{BlockModel, BlockTextures};

#[derive(Component)]
pub struct Chunk(pub [Entity; 16 * 16 * 16]);

pub fn mesh_chunks(
    mut commands: Commands,

    mut meshes: ResMut<Assets<Mesh>>,
    chunks: Query<(Entity, &Chunk), Without<Mesh3d>>,
    blocks: Res<BlockTextures>,
    block_models: Query<&BlockModel>,
) {
    for (entity, chunk) in chunks.iter() {
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        for (i, block) in chunk.0.iter().copied().enumerate() {
            let block_position = Vec3::new(
                (i & 0xF) as f32,
                ((i >> 4) & 0xF) as f32,
                ((i >> 8) & 0xF) as f32,
            );

            let block_model = block_models.get(block).unwrap();
            for j in 0..7 {
                positions.extend(
                    block_model.0[j]
                        .positions
                        .iter()
                        .map(|position| (position + block_position).to_array()),
                );
                normals.extend_from_slice(&block_model.0[j].normals);
                uvs.extend_from_slice(&block_model.0[j].uvs);
            }
        }

        let mut indices = Vec::new();
        for i in (0..positions.len() as u32).step_by(4) {
            indices.extend_from_slice(&[i + 0, i + 1, i + 2, i + 2, i + 3, i + 0]);
        }

        commands.entity(entity).insert((
            Mesh3d(
                meshes.add(
                    Mesh::new(
                        bevy::mesh::PrimitiveTopology::TriangleList,
                        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
                    )
                    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
                    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
                    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
                    .with_inserted_indices(Indices::U32(indices)),
                ),
            ),
            MeshMaterial3d(blocks.material.clone())
        ));
    }
}
