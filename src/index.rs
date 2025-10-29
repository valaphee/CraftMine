use std::hash::Hash;

use bevy::{
    ecs::system::{ReadOnlySystemParam, SystemParam},
    platform::collections::HashMap,
    prelude::*,
};

pub struct Index<'w, T: Send + Sync + Eq + Hash + 'static>(ResMut<'w, IndexStorage<T>>);

impl<'w, T: Send + Sync + Eq + Hash> Index<'w, T> {
    pub fn get(&self, value: &T) -> Option<Entity> {
        self.0.0.get(value).cloned()
    }
}

#[doc(hidden)]
pub struct IndexState<'w, T: Send + Sync + Eq + Hash + 'static>(
    <ResMut<'w, IndexStorage<T>> as SystemParam>::State,
);

unsafe impl<'w, T: Component + Send + Sync + Clone + Eq + Hash + 'static> SystemParam
    for Index<'w, T>
{
    type State = IndexState<'static, T>;

    type Item<'world, 'state> = Index<'world, T>;

    fn init_state(world: &mut World) -> Self::State {
        if !world.contains_resource::<IndexStorage<T>>() {
            world.init_resource::<IndexStorage<T>>();
            world.add_observer(
                |insert: On<Insert, T>, query: Query<&T>, mut storage: ResMut<IndexStorage<T>>| {
                    if let Ok(value) = query.get(insert.entity) {
                        storage.0.retain(|_, entity| *entity != insert.entity);
                        storage.0.insert(value.clone(), insert.entity);
                    }
                },
            );
            world.add_observer(
                |remove: On<Remove, T>, mut storage: ResMut<IndexStorage<T>>| {
                    storage.0.retain(|_, entity| *entity != remove.entity);
                },
            );
        }

        IndexState(<ResMut<'w, IndexStorage<T>> as SystemParam>::init_state(
            world,
        ))
    }

    fn init_access(
        state: &Self::State,
        system_meta: &mut bevy::ecs::system::SystemMeta,
        component_access_set: &mut bevy::ecs::query::FilteredAccessSet,
        world: &mut World,
    ) {
        <ResMut<'w, IndexStorage<T>> as SystemParam>::init_access(
            &state.0,
            system_meta,
            component_access_set,
            world,
        );
    }

    fn apply(
        state: &mut Self::State,
        system_meta: &bevy::ecs::system::SystemMeta,
        world: &mut World,
    ) {
        <ResMut<'w, IndexStorage<T>> as SystemParam>::apply(&mut state.0, system_meta, world);
    }

    unsafe fn get_param<'world, 'state>(
        state: &'state mut Self::State,
        system_meta: &bevy::ecs::system::SystemMeta,
        world: bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell<'world>,
        change_tick: bevy::ecs::change_detection::Tick,
    ) -> Self::Item<'world, 'state> {
        Index(unsafe {
            <ResMut<'w, IndexStorage<T>>>::get_param(&mut state.0, system_meta, world, change_tick)
        })
    }
}

unsafe impl<'w, T: Component + Send + Sync + Clone + Eq + Hash + 'static> ReadOnlySystemParam
    for Index<'w, T>
where
    ResMut<'w, IndexStorage<T>>: ReadOnlySystemParam,
{
}

#[derive(Resource)]
struct IndexStorage<T: Eq + Hash>(HashMap<T, Entity>);

impl<T: Component + Clone + Eq + Hash> FromWorld for IndexStorage<T> {
    fn from_world(world: &mut World) -> Self {
        IndexStorage(
            world
                .query::<(&T, Entity)>()
                .iter(world)
                .map(|(value, entity)| (value.clone(), entity))
                .collect(),
        )
    }
}
