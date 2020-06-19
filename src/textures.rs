use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    prelude::*,
    renderer::{ImageFormat, Texture, SpriteSheetFormat, SpriteRender, SpriteSheet},
};

pub struct SpriteStore {
    handle: Handle<SpriteSheet>
}

impl SpriteStore {
    pub fn from_path(world: &World, name: &str) -> SpriteStore {
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                &format!("texture/{}.png", name),
                ImageFormat::default(),
                (),
                &texture_storage,
            )
        };

        let handle = {
            let loader = world.read_resource::<Loader>();
            let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
            loader.load(
                &format!("texture/{}.ron", name),
                SpriteSheetFormat(texture_handle),
                (),
                &sprite_sheet_store,
            )
        };

        SpriteStore { handle }
    }

    // construct SpriteRender from given sprite_number
    pub fn sprite_renderer(&self, sprite_number: usize) -> SpriteRender {
        SpriteRender {
            sprite_sheet: self.handle.clone(),
            sprite_number: sprite_number,
        }
    }
}

