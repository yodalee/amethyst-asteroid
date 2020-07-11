use amethyst::{
    assets::{Loader},
    ecs::{World},
    ecs::prelude::{Entity},
    prelude::*,
    renderer::{SpriteRender},
    ui::{Anchor, TtfFormat, UiText, UiTransform},
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

pub struct ScoreRes {
    pub score: i32,
    pub text: Entity,
}

impl ScoreRes {
    pub fn initialize(world: &mut World) {
        let font = world.read_resource::<Loader>().load(
            "font/square.ttf",
            TtfFormat,
            (),
            &world.read_resource(),
        );
        let score_transform = UiTransform::new(
            "score".to_string(), Anchor::TopMiddle, Anchor::TopMiddle,
            400., -20., 1., 200., 50.);
        let text = world
            .create_entity()
            .with(score_transform)
            .with(UiText::new(font, "0".to_string(), [0.,0.,0.,1.], 50.))
            .build();

        world.insert(ScoreRes {
            score: 0,
            text: text
        });
    }
}
