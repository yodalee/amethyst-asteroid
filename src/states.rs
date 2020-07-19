use amethyst::{
    core::transform::{Transform},
    core::math::{zero},
    core::ArcThreadPool,
    input::{VirtualKeyCode, is_key_down},
    prelude::*,
    renderer::{Camera},
    shred::{Dispatcher, DispatcherBuilder},
};

use crate::components::{
    Ship,
    Physical,
    Asteroid,
    Bullet,
    Collider,
    ColliderType
};
use crate::resources::{
    ShipRes,
    BulletRes,
    AsteroidRes,
    RandomGen,
    ExplosionRes,
    ScoreRes
};
use crate::system::{
    ShipControlSystem,
    PhysicalSystem,
    BoundarySystem,
    SpawnAsteroidSystem,
    CollisionSystem,
    ExplosionSystem,
};

pub const ARENA_HEIGHT: f32 = 300.0;
pub const ARENA_WIDTH: f32 = 300.0;

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

#[derive(Default)]
pub struct AsteroidGame<'a, 'b> {
    pub dispatcher: Option<Dispatcher<'a, 'b>>,
}
pub struct AsteroidGamePause;

impl<'a, 'b> SimpleState for AsteroidGame<'a, 'b> {
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
        initialize_ship(world);

        // create dispatcher
        let mut dispatcher = DispatcherBuilder::new()
            .with(ShipControlSystem, "ship_control_system", &[])
            .with(PhysicalSystem, "physical_system", &["ship_control_system"])
            .with(BoundarySystem, "boundary_system", &["physical_system"])
            .with(SpawnAsteroidSystem::new(), "spawn_system", &[])
            .with(CollisionSystem, "collision_system", &[])
            .with(ExplosionSystem, "explosion_system", &[])
            .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
            .build();
        dispatcher.setup(world);

        self.dispatcher = Some(dispatcher);
    }

    fn handle_event(&mut self,
                    _data: StateData<'_, GameData<'_, '_>>,
                    event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                println!("Escape pressed");
                return Trans::Push(Box::new(AsteroidGamePause));
            }
        }

        Trans::None
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }

        Trans::None
    }
}

impl SimpleState for AsteroidGamePause {
    fn handle_event(&mut self,
                    _data: StateData<'_, GameData<'_, '_>>,
                    event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Pop;
            }
        }

        Trans::None
    }
}
