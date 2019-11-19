mod audio;
mod pong;
mod systems;

use crate::audio::Music;
use crate::pong::Pong;

use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    ui::{RenderUi, UiBundle},
    audio::{AudioBundle, DjSystemDesc},
    utils::application_root_dir,
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    
    let app_root = application_root_dir()?;
    let display_config = app_root.join("config").join("display.ron");

    let assets_dir = app_root.join("assets");

    // Application setup
    let mut world = World::new();

    let game_data = GameDataBuilder::default()
        // Add the transform bundle which handles tracking entity positions
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(
                app_root.join("config").join("bindings.ron"),
            )?,
        )?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(AudioBundle::default())?
        .with_system_desc(DjSystemDesc::new(|music: &mut Music| music.music.next()),
            "dj_system",
            &[],
        )
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::MoveBallSystem, "ball_system", &[])
        .with(systems::BounceSystem, "collision_system", &["paddle_system", "ball_system"])
        .with(systems::WinnerSystem, "winner_system", &["ball_system"])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
            // The RenderToWindow plugin provides all the scaffolding for opening a window and
            // drawing it
            .with_plugin(
                RenderToWindow::from_config_path(display_config)
                    .with_clear([0.0, 0.0, 0.0, 1.0]),
            )
            // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
            .with_plugin(RenderFlat2D::default())
            .with_plugin(RenderUi::default()),
        )?;

    let mut game = Application::new(assets_dir, Pong::default(), game_data)?;
    game.run();

    Ok(())
}

