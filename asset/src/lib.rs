use bevy_reflect::Reflect;
use serde::{Deserialize, Serialize};

pub mod model;

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
