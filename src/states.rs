use amethyst::{
    core::transform::{Transform},
    core::math::{zero},
    prelude::*,
    renderer::{Camera},
};

use crate::components::{Ship, Physical};
use crate::textures::{SpriteStore};

pub const ARENA_HEIGHT: f32 = 300.0;
pub const ARENA_WIDTH: f32 = 300.0;

pub struct Asteroid;

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(transform)
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .build();
}

fn initialize_ship(world: &mut World, sprite_handle: SpriteStore) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 0.0);

    let sprite_render = sprite_handle.sprite_renderer(0);

    world
        .create_entity()
        .with(transform)
        .with(sprite_render.clone())
        .with(Ship::new())
        .with(Physical {
            velocity: zero(),
            max_velocity: 100.0,
            rotation: 0.0
        })
        .build();
}

impl SimpleState for Asteroid {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet = SpriteStore::from_path(world, "ship");

        world.register::<Ship>();

        initialize_camera(world);
        initialize_ship(world, sprite_sheet);
    }
}

