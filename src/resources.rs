use amethyst::{
    ecs::{World},
    renderer::{SpriteRender},
};

use rand;
use crate::textures::{SpriteStore};

pub struct ShipRes {
    pub sprite_store: SpriteStore,
}

impl ShipRes {
    pub fn initialize(world: &mut World) {
        let sprite_store = SpriteStore::from_path(world, "ship");
        world.insert(
            ShipRes { sprite_store : sprite_store }
        );
    }

    pub fn sprite_render(&self) -> SpriteRender {
        self.sprite_store.sprite_renderer(0)
    }
}

pub struct BulletRes {
    pub sprite_store: SpriteStore,
}

impl BulletRes {
    pub fn initialize(world: &mut World) {
        let sprite_store = SpriteStore::from_path(world, "bullet");
        world.insert(
            BulletRes { sprite_store: sprite_store }
        );
    }

    pub fn sprite_render(&self) -> SpriteRender {
        self.sprite_store.sprite_renderer(0)
    }
}

pub struct AsteroidRes {
    pub sprite_store: SpriteStore,
}

impl AsteroidRes {
    pub fn initialize(world: &mut World) {
        let sprite_store = SpriteStore::from_path(world, "asteroids");
        world.insert(
            AsteroidRes { sprite_store: sprite_store }
        );
    }

    pub fn sprite_render(&self) -> SpriteRender {
        self.sprite_store.sprite_renderer(0)
    }
}

pub struct RandomGen;

impl RandomGen {
    pub fn next_f32(&self) -> f32 {
        use rand::Rng;
        rand::thread_rng().gen::<f32>()
    }
}

pub struct ExplosionRes {
    pub sprite_store: SpriteStore,
}

impl ExplosionRes {
    pub fn initialize(world: &mut World) {
        let sprite_store = SpriteStore::from_path(world, "explosion");
        world.insert(
            ExplosionRes { sprite_store: sprite_store }
        );
    }

    pub fn sprite_render(&self) -> SpriteRender {
        self.sprite_store.sprite_renderer(0)
    }
}
