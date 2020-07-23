use amethyst::{
    core::transform::{Transform},
    input::{VirtualKeyCode, is_key_down},
    prelude::*,
    renderer::{Camera},
};

use crate::components::{
    Ship,
    Physical,
    Asteroid,
    Bullet,
    Collider,
};
use crate::resources::{
    ShipRes,
    BulletRes,
    AsteroidRes,
    RandomGen,
    ExplosionRes,
    ScoreRes
};
use crate::states::{
    ARENA_WIDTH, ARENA_HEIGHT,
    StatePlay,
};

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(transform)
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .build();
}

#[derive(Default)]
pub struct StateMenu;

impl SimpleState for StateMenu {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        ShipRes::initialize(world);
        BulletRes::initialize(world);
        AsteroidRes::initialize(world);
        ExplosionRes::initialize(world);
        ScoreRes::initialize(world);
        world.insert(RandomGen);

        world.register::<Physical>();
        world.register::<Ship>();
        world.register::<Bullet>();
        world.register::<Asteroid>();
        world.register::<Collider>();

        initialize_camera(world);
    }

    fn handle_event(&mut self,
                    _data: StateData<'_, GameData<'_, '_>>,
                    event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = event {
            if is_key_down(&event, VirtualKeyCode::Space) {
                println!("Space pressed");
                return Trans::Push(Box::new(StatePlay::default()));
            }
        }

        Trans::None
    }
}
