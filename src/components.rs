use amethyst::{
    core::math::Vector2,
    ecs::prelude::{Component, DenseVecStorage},
};

pub struct Physical {
    // velocity, [vx, vy]
    pub velocity: Vector2<f32>,
    // maximum velocity (units / s)
    pub max_velocity: f32,
    // rotation (rad / s)
    pub rotation: f32,
}

impl Component for Physical {
    type Storage = DenseVecStorage<Self>;
}

pub struct Ship {
    pub acceleration: f32,
    pub rotate: f32,
    pub reload_timer: f32,
    pub time_to_reload: f32,
}

impl Ship {
    pub fn new() -> Self {
        Self {
            acceleration: 80f32,
            rotate: 180f32,
            // time to reload
            reload_timer: 0.0f32,
            time_to_reload: 0.5f32,
        }
    }
}

impl Component for Ship {
    type Storage = DenseVecStorage<Self>;
}

pub struct Bullet;

impl Component for Bullet {
    type Storage = DenseVecStorage<Self>;
}

pub struct Asteroid;

impl Component for Asteroid {
    type Storage = DenseVecStorage<Self>;
}

pub struct Explosion {
    pub time_to_update: f32,
    pub frame_count: i32,
}

impl Explosion {
    pub const FRAME_LIMIT: i32 = 4;
    pub fn new() -> Self {
        Self {
            time_to_update: 0.1f32,
            frame_count: 0,
        }
    }
}

impl Component for Explosion {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone)]
pub struct Collider {
    pub typ: ColliderType
}

#[derive(PartialEq,Clone,Copy)]
pub enum ColliderType {
    Ship,
    Bullet,
    Asteroid,
}

impl Component for Collider {
    type Storage = DenseVecStorage<Self>;
}
