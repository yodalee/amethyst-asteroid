use amethyst::{
    core::{
        math::{Vector3, Vector2, zero, Isometry2},
        transform::components::Transform,
        timing::Time,
    },
    derive::{SystemDesc},
    ecs::{Join,
          ReadStorage, WriteStorage,
          System, SystemData,
          Read, ReadExpect,
          Entities, LazyUpdate,
          Write, WriteExpect,
          World},
    ecs::prelude::{Entity},
    prelude::{Trans, TransEvent, GameData, StateEvent},
    renderer::{SpriteRender},
    shrev::{EventChannel, ReaderId},
    input::{InputHandler, StringBindings},
    ui::{UiText},
};

use log::{error};
use ncollide2d::{
    bounding_volume,
    shape::Ball,
    broad_phase::{DBVTBroadPhase, BroadPhase, BroadPhaseInterferenceHandler}};

use crate::components::{Physical, Ship, Bullet, Asteroid, Explosion, Collider, ColliderType};
use crate::resources::{BulletRes, AsteroidRes, RandomGen, ExplosionRes, ScoreRes};
use crate::states::{ARENA_WIDTH, ARENA_HEIGHT, self};

#[derive(SystemDesc)]
pub struct ShipControlSystem;

impl<'s> System<'s> for ShipControlSystem {
    type SystemData = (
        WriteStorage<'s, Physical>,
        WriteStorage<'s, Ship>,
        ReadStorage<'s, Transform>,
        ReadExpect<'s, BulletRes>,
        Entities<'s>,
        Read<'s, LazyUpdate>,
        Read<'s, InputHandler::<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self,
           (mut physicals,
            mut ships,
            transforms,
            bullet_resources,
            entities,
            lazy,
            input,
            time): Self::SystemData) {
        let delta = time.delta_seconds();

        for (physical, ship, transform) in (&mut physicals, &mut ships, &transforms).join() {
            let acceleration = input.axis_value("accelerate");
            let rotate = input.axis_value("rotate");
            let shoot = input.action_is_down("shoot").unwrap_or(false);

            // handle acceleration -> velocity
            let acc = acceleration.unwrap_or_default();
            let added = Vector3::y() * delta * acc * ship.acceleration;
            let added = transform.rotation() * added;
            physical.velocity += Vector2::new(added.x, added.y);

            let magnitude = physical.velocity.magnitude();
            if magnitude > physical.max_velocity {
                physical.velocity *= physical.max_velocity / magnitude;
            }

            // handle rotation -> rotate
            physical.rotation = rotate.unwrap_or_default() * delta * ship.rotate;

            // handle shoot
            if ship.reload_timer <= 0.0f32 {
                if shoot {
                    ship.reload_timer = ship.time_to_reload;

                    let bullet_transform = transform.clone();
                    let velocity = transform.rotation() * Vector3::y() * 150f32;
                    let velocity = physical.velocity + Vector2::new(velocity.x, velocity.y);
                    let bullet_physical = Physical {
                        velocity: velocity,
                        max_velocity: 200f32,
                        rotation: 0f32,
                    };

                    let e = entities.create();

                    lazy.insert(e, Bullet {} );
                    lazy.insert(e, bullet_transform);
                    lazy.insert(e, bullet_physical);
                    lazy.insert(e, Collider { typ: ColliderType::Bullet } );
                    lazy.insert(e, bullet_resources.sprite_render());
                }
            } else {
                ship.reload_timer = (ship.reload_timer - delta).max(0.0f32);
            }
        }
    }
}

#[derive(SystemDesc)]
pub struct PhysicalSystem;

impl<'s> System<'s> for PhysicalSystem {
    type SystemData = (
        ReadStorage<'s, Physical>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self,
           (physicals,
            mut transforms,
            time): Self::SystemData) {
        let delta = time.delta_seconds();
        for (physical, transform) in (&physicals, &mut transforms).join() {
            let movement = physical.velocity * delta;
            let rotation = physical.rotation * delta;
            transform.prepend_translation(Vector3::new(movement.x, movement.y, 0.0));
            transform.rotate_2d(rotation);
        }

    }
}

#[derive(SystemDesc)]
pub struct BoundarySystem;

impl<'s> System<'s> for BoundarySystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Physical>,
        ReadStorage<'s, Bullet>,
        Entities<'s>,
    );

    fn run(&mut self,
           (mut transforms,
            physicals,
            bullets,
            entities): Self::SystemData) {
        for (_, _, transform) in (&physicals, !&bullets, &mut transforms).join() {
            let ship_x = transform.translation().x;
            let ship_y = transform.translation().y;
            if ship_x < 0.0 {
                transform.set_translation_x(ARENA_WIDTH-0.5);
            } else if ship_x > ARENA_WIDTH {
                transform.set_translation_x(0.5);
            }

            if ship_y < 0.0 {
                transform.set_translation_y(ARENA_HEIGHT-0.5);
            } else if ship_y > ARENA_HEIGHT {
                transform.set_translation_y(0.5);
            }
        }

        for (e, _, transform) in (&*entities, &bullets, &mut transforms).join() {
            let x = transform.translation().x;
            let y = transform.translation().y;
            if x < 0.0 || y < 0.0 || x > ARENA_WIDTH || y > ARENA_HEIGHT {
                if let Err(e) = entities.delete(e) {
                    error!("Failed to destroy entity: {}", e);
                }

                continue;
            }
        }
    }
}

#[derive(SystemDesc)]
pub struct SpawnAsteroidSystem {
    pub time_to_spawn: f32,
    pub max_velocity: f32,
    pub max_rotation: f32,
    pub distance_to_ship: f32,
    pub average_spawn_time: f32,
}

impl SpawnAsteroidSystem {
    pub fn new() -> Self {
        Self {
            time_to_spawn: 2f32,
            max_velocity: 60f32,
            max_rotation: 5f32,
            distance_to_ship: 200f32,
            average_spawn_time: 0.5f32,
        }
    }
}

impl<'s> System<'s> for SpawnAsteroidSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Ship>,
        ReadStorage<'s, Transform>,
        ReadExpect<'s, AsteroidRes>,
        ReadExpect<'s, RandomGen>,
        Read<'s, LazyUpdate>,
        Read<'s, Time>,
    );

    fn run(&mut self,
           (entities,
            ships,
            transforms,
            asteroidres,
            rand,
            lazy,
            time): Self::SystemData) {
        let delta = time.delta_seconds();
        self.time_to_spawn -= delta;

        if self.time_to_spawn <= 0.0f32 {
            for (_, ship_transform) in (&ships, &transforms).join()  {
                let ship_translation = ship_transform.translation();

                let mut transform = Transform::default();
                let mut create_point: Vector3<f32> = zero();
                // generate creation point
                loop {
                    create_point.x = rand.next_f32() * ARENA_WIDTH;
                    create_point.y = rand.next_f32() * ARENA_HEIGHT;
                    if (ship_translation-create_point).norm() > self.distance_to_ship {
                        break;
                    }
                }
                transform.set_translation_x(create_point.x);
                transform.set_translation_y(create_point.y);
                // scale, velocity, rotation
                transform.set_scale(Vector3::new(1.0, 1.0, 1.0));
                let gen = || (rand.next_f32() - 0.5) * 2.0 * self.max_velocity;
                let velocity = Vector2::new(gen(), gen());

                let physical = Physical {
                    velocity: velocity,
                    max_velocity: 100f32,
                    rotation: self.max_rotation * 2.0 * (rand.next_f32() - 0.5),
                };

                let e = entities.create();

                lazy.insert(e, Asteroid {} );
                lazy.insert(e, transform);
                lazy.insert(e, physical);
                lazy.insert(e, Collider { typ: ColliderType::Asteroid } );
                lazy.insert(e, asteroidres.sprite_render(
                        (rand.next_u32() % 3) as usize));
                self.time_to_spawn = self.average_spawn_time + rand.next_f32();
            }
        }
    }
}

struct CollisionHandler {
    collide_entity: Vec<Entity>,
    ship_hit: bool,
}

impl CollisionHandler {
    pub fn new() -> Self {
        Self {
            collide_entity: vec![],
            ship_hit: false,
        }
    }
}

type ColliderEntity = (ColliderType, Entity);

impl BroadPhaseInterferenceHandler<ColliderEntity> for CollisionHandler {
    fn is_interference_allowed(&mut self, a: &ColliderEntity, b: &ColliderEntity) -> bool {

        a.0 != b.0
    }
    fn interference_started(&mut self, a: &ColliderEntity, b: &ColliderEntity) {
        match (a.0, b.0) {
            (ColliderType::Asteroid, ColliderType::Bullet) |
            (ColliderType::Bullet, ColliderType::Asteroid) => {
                self.collide_entity.push(a.1);
                self.collide_entity.push(b.1);
            },
            (ColliderType::Asteroid, ColliderType::Ship) |
            (ColliderType::Ship, ColliderType::Asteroid) => {
                self.ship_hit = true;
            },
            (_, _) => {
            }
        }
    }
    fn interference_stopped(&mut self, _a: &ColliderEntity, _b: &ColliderEntity) {
    }
}

#[derive(Debug)]
pub struct CollisionEvent {
    pub entity: Entity,
}

impl CollisionEvent {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity: entity
        }
    }
}

#[derive(SystemDesc)]
pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Collider>,
        ReadStorage<'s, Transform>,
        Write<'s, EventChannel<TransEvent<GameData<'static, 'static>, StateEvent>>>,
        Write<'s, EventChannel<CollisionEvent>>,
    );
    fn run(&mut self,
           (entities,
            colliders,
            transforms,
            mut trans_events,
            mut collision_channel): Self::SystemData) {

        // collect collider
        let mut broad_phase = DBVTBroadPhase::new(0f32);
        let mut handler = CollisionHandler::new();
        //let mut vec = vec![];
        for (e, collider, transform) in (&entities, &colliders, &transforms).join()  {
            let pos = transform.translation();
            let pos = Isometry2::new(Vector2::new(pos.x, pos.y), zero());
            let vol = bounding_volume::bounding_sphere( &Ball::new(5.0), &pos );
            broad_phase.create_proxy(vol, (collider.typ, e));
        }

        broad_phase.update(&mut handler);

        if handler.ship_hit {
            let trans = Box::new(
                move || Trans::Switch(Box::new(states::StateOver::new())));
            trans_events.single_write(trans);
        }

        for e in handler.collide_entity {
            collision_channel.single_write(CollisionEvent::new(e));
        }
    }
}

#[derive(Default)]
pub struct DeletionSystem {
    event_reader: Option<ReaderId<CollisionEvent>>,
}

impl<'s> System<'s> for DeletionSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Collider>,
        ReadStorage<'s, Transform>,
        Read<'s, EventChannel<CollisionEvent>>,
        ReadExpect<'s, ExplosionRes>,
        WriteStorage<'s, UiText>,
        WriteExpect<'s, ScoreRes>,
        Read<'s, LazyUpdate>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<CollisionEvent>>()
                .register_reader()
        )
    }

    fn run(&mut self,
           (entities,
            colliders,
            transforms,
            collision_channel,
            explosionres,
            mut uitext,
            mut scoretexts,
            lazy): Self::SystemData) {
        for event in collision_channel.read(self.event_reader.as_mut().unwrap()) {
            let e = event.entity;
            if let Some(c) = colliders.get(e) {
                if c.typ == ColliderType::Bullet {
                    // create explosion
                    if let Some(trans) = transforms.get(e) {
                        let e = entities.create();
                        lazy.insert(e, Explosion::new() );
                        lazy.insert(e, trans.clone());
                        lazy.insert(e, explosionres.sprite_render());
                    }

                    // change score
                    scoretexts.score = scoretexts.score + 1;
                    if let Some(text) = uitext.get_mut(scoretexts.text) {
                        text.text = scoretexts.score.to_string()
                    }
                }
            }

            // delete the bullet and asteroid
            if let Err(e) = entities.delete(e) {
                error!("Failed to destroy collide entity: {}", e)
            }
        }
    }
}

#[derive(SystemDesc)]
pub struct ExplosionSystem;

impl<'s> System<'s> for ExplosionSystem {
    type SystemData = (
        WriteStorage<'s, Explosion>,
        WriteStorage<'s, SpriteRender>,
        Entities<'s>,
        Read<'s, Time>,
    );

    fn run(&mut self,
           (mut explosions,
            mut spriterenders,
            entities,
            time): Self::SystemData) {
        let delta = time.delta_seconds();

        for (e, explosion, spriterender) in (&*entities, &mut explosions, &mut spriterenders).join() {
            if explosion.time_to_update <= 0.0 {
                if explosion.frame_count == Explosion::FRAME_LIMIT {
                    if let Err(e) = entities.delete(e) {
                        error!("Failed to destroy explosion: {}", e)
                    }
                } else {
                    spriterender.sprite_number += 1;
                    explosion.frame_count += 1;
                }
            } else {
                explosion.time_to_update -= delta;
            }
        }
    }
}
