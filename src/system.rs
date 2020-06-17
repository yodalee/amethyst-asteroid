use amethyst::{
    core::{
        math::{Vector3, Vector2},
        transform::components::Transform,
        timing::Time,
    },
    derive::{SystemDesc},
    ecs::{Join, ReadStorage, WriteStorage, System, SystemData, Read},
    input::{InputHandler, StringBindings},
};

use crate::components::{Ship, Physical};

#[derive(SystemDesc)]
pub struct ShipControlSystem;

impl<'s> System<'s> for ShipControlSystem {
    type SystemData = (
        WriteStorage<'s, Physical>,
        ReadStorage<'s, Ship>,
        ReadStorage<'s, Transform>,
        Read<'s, InputHandler::<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self,
           (mut physicals,
            ships,
            transforms,
            input,
            time): Self::SystemData) {
        let delta = time.delta_seconds();

        for (physical, ship, transform) in (&mut physicals, &ships, &transforms).join() {
            let acceleration = input.axis_value("accelerate");
            let rotate = input.axis_value("rotate");

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
