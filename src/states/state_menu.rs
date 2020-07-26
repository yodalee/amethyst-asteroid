use amethyst::{
    assets::{Loader},
    core::transform::{Transform},
    ecs::prelude::{Entity},
    input::{VirtualKeyCode, is_key_down},
    prelude::*,
    renderer::{Camera},
    ui::{Anchor, TtfFormat, UiText, UiTransform},
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

fn initialize_text(world: &mut World) -> Entity {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let score_transform = UiTransform::new(
        "score".to_string(), Anchor::Middle, Anchor::Middle,
        0., 0., 1., 1000., 50.);
    let text = world
        .create_entity()
        .with(score_transform)
        .with(UiText::new(font,
                          "Press Space to Start".to_string(),
                          [0.,0.,0.,1.], 50.))
        .build();
    text
}

#[derive(Default)]
pub struct StateMenu {
    text: Option<Entity>,
}

impl SimpleState for StateMenu {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        ShipRes::initialize(world);
        BulletRes::initialize(world);
        AsteroidRes::initialize(world);
        ExplosionRes::initialize(world);
        world.insert(RandomGen);

        world.register::<Physical>();
        world.register::<Ship>();
        world.register::<Bullet>();
        world.register::<Asteroid>();
        world.register::<Collider>();

        initialize_camera(world);
        let text = initialize_text(world);
        self.text = Some(text);
    }

    fn on_pause(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        if let Some(text) = self.text {
            if let Err(e) = world.delete_entity(text) {
                log::error!("Cannot delete menu ui text: {}", e);
            }
        }
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
