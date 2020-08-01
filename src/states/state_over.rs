use amethyst::{
    ecs::prelude::{Entity},
    input::{VirtualKeyCode, is_key_down},
    prelude::*,
    ui::{Anchor, UiText, UiTransform},
};
use crate::resources::{FontRes};

#[derive(Default)]
pub struct StateOver {
    text: Option<Entity>,
}

impl StateOver {
    pub fn new() -> Self {
        Default::default()
    }
}

fn initialize_text(world: &mut World) -> Entity {
    let font = world.read_resource::<FontRes>().font();
    let score_transform = UiTransform::new(
        "score".to_string(), Anchor::Middle, Anchor::Middle,
        0., 0., 1., 1000., 50.);
    let text = world
        .create_entity()
        .with(score_transform)
        .with(UiText::new(font,
                          "Game Over".to_string(),
                          [0., 0., 0., 1.], 50.))
        .build();
    text
}

impl SimpleState for StateOver {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let text = initialize_text(world);
        self.text = Some(text)
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.delete_all();
    }

    fn handle_event(&mut self,
                    _data: StateData<'_, GameData<'_, '_>>,
                    event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = event {
            if is_key_down(&event, VirtualKeyCode::Space) {
                return Trans::Pop;
            }
        }

        Trans::None
    }
}
