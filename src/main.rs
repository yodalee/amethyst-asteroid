use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod states;
mod components;
mod system;
mod textures;
mod resources;

use crate::states::AsteroidGame;
use crate::system::{
    ShipControlSystem,
    PhysicalSystem,
    BoundarySystem,
    SpawnAsteroidSystem,
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let input_config_path = config_dir.join("input.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(input_config_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(ShipControlSystem, "ship_control_system", &["input_system"])
        .with(PhysicalSystem, "physical_system", &["ship_control_system"])
        .with(BoundarySystem, "boundary_system", &["physical_system"])
        .with(SpawnAsteroidSystem::new(), "spawn_system", &[]);

    let mut game = Application::new(assets_dir, AsteroidGame, game_data)?;
    game.run();

    Ok(())
}
