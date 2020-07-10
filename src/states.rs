use amethyst::{
    assets::{Loader},
    core::transform::{Transform},
    core::math::{zero},
    prelude::*,
    renderer::{Camera},
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

use crate::components::{Ship, Physical, Asteroid, Collider, ColliderType};
use crate::resources::{
    ShipRes,
    BulletRes,
    AsteroidRes,
    RandomGen,
    ExplosionRes,
    ScoreTextRes
};

pub const ARENA_HEIGHT: f32 = 300.0;
pub const ARENA_WIDTH: f32 = 300.0;

pub struct AsteroidGame;

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(transform)
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .build();
}

fn initialize_ship(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 0.0);

    let sprite_render = {
        let resource = world.read_resource::<ShipRes>();
        resource.sprite_render()
    };

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
        .with(Collider { typ: ColliderType::Ship })
        .build();
}

fn initialize_score(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let score_transform = UiTransform::new(
        "score".to_string(), Anchor::TopMiddle, Anchor::TopMiddle,
        400., -20., 1., 200., 50.);
    let score = world
        .create_entity()
        .with(score_transform)
        .with(UiText::new(font, "0".to_string(), [0.,0.,0.,1.], 50.))
        .build();

    world.insert(ScoreTextRes { score } );
}

impl SimpleState for AsteroidGame {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        ShipRes::initialize(world);
        BulletRes::initialize(world);
        AsteroidRes::initialize(world);
        ExplosionRes::initialize(world);
        world.insert(RandomGen);

        world.register::<Asteroid>();

        initialize_camera(world);
        initialize_ship(world);
        initialize_score(world);
    }
}

