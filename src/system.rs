use amethyst::{
    core::{Transform},
    derive::{SystemDesc},
    ecs::{Join, ReadStorage, WriteStorage, System, SystemData, Read},
    input::{InputHandler, StringBindings},
};

use crate::entities::{Ship};

#[derive(SystemDesc)]
pub struct ShipControlSystem;

impl<'s> System<'s> for ShipControlSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Ship>,
        Read<'s, InputHandler::<StringBindings>>,
    );

    fn run(&mut self,
           (mut transforms,
            ships,
            input): Self::SystemData) {
        for (ship, transform) in (&ships, &mut transforms).join() {
            let rotate = input.axis_value("rotate");
            if let Some(rotate) = rotate {
                println!("{}", rotate);
            }
        }
    }
}
