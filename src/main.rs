use crate::sub_systems::config_manager::ConfigManager;
use fast_log::filter::ModuleFilter;
use ggez::event;
use ggez::glam::*;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use play_states::initialization_state::InitializationState;
use play_states::play_state::PlayState;
use play_states::title_state::TitleState;
use std::env;
use std::path;

mod play_states;
mod sub_systems;

pub struct SubSystems {
    pub config_manager: ConfigManager,
}

pub struct GameData {}

pub struct GameState {
    sub_systems: SubSystems,
    game_data: GameData,
    state: Option<Box<dyn PlayState>>,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        let config_manager = ConfigManager::default();

        Ok(GameState {
            sub_systems: SubSystems { config_manager },
            game_data: GameData {},
            state: Some(Box::<InitializationState>::default()),
        })
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // if ctx.time.ticks() % 100 == 0 {
        //     info!("Delta frame time: {:?} ", ctx.time.delta());
        //     info!("Average FPS: {}", ctx.time.fps());
        // }
        if let Some(play_state) = self.state.take() {
            self.state = Some(play_state.update(&mut self.sub_systems, ctx));
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        self.state
            .as_ref()
            .unwrap()
            .draw(&mut self.game_data, &mut canvas);
        // canvas.draw(
        //     &self.image,
        //     graphics::DrawParam::new().dest(Point2::new(0.000, 0.000)),
        // );

        canvas.finish(ctx)?;
        // We yield the current thread until the next update
        ggez::timer::yield_now();
        // And return success.
        Ok(())
    }
}

pub fn main() -> GameResult {
    let _ = std::fs::remove_file("./game.log");
    fast_log::init(
        fast_log::Config::new()
            .console()
            .file("./game.log")
            .chan_len(Some(100000))
            .filter(ModuleFilter {
                exclude: None,
                include: Some(vec!["rpg_game_rust".to_string()]),
            })
            .level(log::LevelFilter::Info),
    )
    .expect("Failed to initialize logging subsystem");

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./delete_later_resources")
    };

    let cb = ggez::ContextBuilder::new("RPG Game Rust", "Arthur Cruz")
        .add_resource_path(resource_dir)
        .window_setup(ggez::conf::WindowSetup::default().title("RPG Game Rust"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(600.0, 400.0));
    let (mut ctx, event_loop) = cb.build()?;

    let state = GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
