use amethyst::{
    ecs::{World},
    renderer::{SpriteRender},
};

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
