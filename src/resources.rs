use amethyst::{
    prelude::*,
    ecs::{World},
    renderer::{SpriteRender},
};

use crate::textures::{SpriteStore};

pub struct Ships {
    pub sprite_store: SpriteStore,
}

impl Ships {
    pub fn initialize(world: &mut World) {
        let sprite_store = SpriteStore::from_path(world, "ship");
        world.insert(
            Ships { sprite_store : sprite_store }
        );
    }

    pub fn sprite_render(&self) -> SpriteRender {
        self.sprite_store.sprite_renderer(0)
    }
}

pub struct Bullets {
    pub sprite_store: SpriteStore,
}

impl Bullets {
    pub fn initialize(world: &mut World) {
        let sprite_store = SpriteStore::from_path(world, "bullet");
        world.insert(
            Bullets { sprite_store: sprite_store }
        );
    }

    pub fn sprite_render(&self) -> SpriteRender {
        self.sprite_store.sprite_renderer(0)
    }
}
