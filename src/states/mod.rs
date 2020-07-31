mod state_play;
mod state_pause;
mod state_menu;
mod state_over;

pub const ARENA_HEIGHT: f32 = 300.0;
pub const ARENA_WIDTH: f32 = 300.0;

pub use state_play::StatePlay;
pub use state_pause::StatePause;
pub use state_menu::StateMenu;
pub use state_over::StateOver;
