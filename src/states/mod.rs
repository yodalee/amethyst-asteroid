mod state_play;
mod state_pause;

pub const ARENA_HEIGHT: f32 = 300.0;
pub const ARENA_WIDTH: f32 = 300.0;

pub use state_play::StatePlay;
pub use state_pause::StatePause;
