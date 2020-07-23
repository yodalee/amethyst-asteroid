use amethyst::{
    input::{VirtualKeyCode, is_key_down},
    prelude::*,
};

pub struct StatePause;

impl SimpleState for StatePause {
    fn handle_event(&mut self,
                    _data: StateData<'_, GameData<'_, '_>>,
                    event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Pop;
            }
        }

        Trans::None
    }
}
