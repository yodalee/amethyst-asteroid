use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::{Transform},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteSheet, Texture, SpriteSheetFormat, SpriteRender},
};

use crate::entities::{Ship};

const ARENA_HEIGHT: f32 = 300.0;
const ARENA_WIDTH: f32 = 300.0;

pub struct Asteroid;

fn load_sprite_sheet(world: &World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/ship.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/ship.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(transform)
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .build();
}

fn initialize_ship(world: &mut World, sprite_handle: Handle<SpriteSheet>) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_handle,
        sprite_number: 0
    };

    world
        .create_entity()
        .with(transform)
        .with(sprite_render.clone())
        .with(Ship::new())
        .build();
}

impl SimpleState for Asteroid {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet = load_sprite_sheet(world);

        world.register::<Ship>();

        initialize_camera(world);
        initialize_ship(world, sprite_sheet);
    }
}

