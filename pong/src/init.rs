use crate::state;
use amethyst::{
    core::TransformBundle,
    input::StringBindings,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

pub fn init<'a>() -> amethyst::Result<
    CoreApplication<
        'a,
        GameData<'static, 'static>,
        StateEvent<StringBindings>,
        amethyst::StateEventReader,
    >,
> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0., 0., 0., 1.]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?;

    let assets_dir = app_root.join("assets");
    let game = Application::new(assets_dir, state::Pong, game_data)?;
    Ok(game)
}
