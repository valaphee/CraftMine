use std::collections::HashMap;

use bevy::prelude::*;

#[derive(Component)]
struct World(HashMap<IVec2, Vec<Chunk>>);

struct Chunk([Entity; 16 * 16 * 16]);
