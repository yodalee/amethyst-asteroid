use amethyst::{
    core::{
        math::{Vector3, Vector2},
        transform::components::Transform,
        timing::Time,
    },
    derive::{SystemDesc},
    ecs::{Join, ReadStorage, WriteStorage, System, SystemData, Read, ReadExpect, Entities, LazyUpdate},
    input::{InputHandler, StringBindings},
};

use log::{error};

use crate::components::{Physical, Ship, Bullet};
use crate::resources::{BulletRes};
use crate::states::{ARENA_WIDTH, ARENA_HEIGHT};

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
        ReadStorage<'s, Ship>,
        ReadStorage<'s, Bullet>,
        Entities<'s>,
    );

    fn run(&mut self,
           (mut transforms,
            physicals,
            ships,
            bullets,
            entities): Self::SystemData) {
        for (_physical, _ships, transform) in (&physicals, &ships, &mut transforms).join() {
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
                    error!("Failed to destroy entity: {}", e)
                }

                continue;
            }
        }
    }
}
