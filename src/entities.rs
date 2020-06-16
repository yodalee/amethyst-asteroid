use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
};


pub struct Ship {
    velocity: [f32;2],
}

impl Ship {
    pub fn new() -> Self {
        Self {
            velocity: [0.0, 0.0],
        }
    }
}

impl Component for Ship {
    type Storage = DenseVecStorage<Self>;
}
