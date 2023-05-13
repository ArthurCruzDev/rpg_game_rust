use std::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

use ggez::{
    graphics::{self, Color, Text},
    Context,
};

use crate::{sub_systems::config_manager::ConfigManager, GameData, SubSystems};

use super::play_state::PlayState;

pub struct InitializationState {
    current_step: u32,
    total_steps: u32,
    currently_loading: String,
    config_manager_loaded: Option<bool>,
    loaded_config_manager: Arc<Mutex<Option<ConfigManager>>>,
    receiver: Receiver<()>,
    sender: Sender<()>,
}

impl InitializationState {
    // pub fn new() -> InitializationState {
    //     InitializationState {
    //         current_step: 0,
    //         total_steps: 10,
    //     }
    // }
    // pub fn update(&mut self, game_state: &mut GameState, ctx: &mut Context) {}
}

impl Default for InitializationState {
    fn default() -> Self {
        let (tx, rx) = channel();
        Self {
            current_step: 0,
            total_steps: 10,
            currently_loading: "".to_string(),
            config_manager_loaded: None,
            loaded_config_manager: Arc::new(Mutex::new(None)),
            receiver: rx,
            sender: tx,
        }
    }
}

impl PlayState for InitializationState {
    fn update(
        mut self: Box<Self>,
        sub_systems: &mut SubSystems,
        ctx: &mut Context,
    ) -> Box<dyn PlayState> {
        match self.config_manager_loaded {
            None => {
                self.currently_loading = "Configuration file".to_string();
                let (data, sender) = (Arc::clone(&self.loaded_config_manager), self.sender.clone());
                thread::spawn(move || {
                    let mut data = data.lock().unwrap();
                    *data = Some(ConfigManager::read_configuration_from_file());
                    sender.send(()).unwrap();
                });
                self.config_manager_loaded = Some(false);
                return self;
            }
            Some(loaded) => match loaded {
                false => match self.receiver.try_recv() {
                    Ok(_boolean) => {
                        self.config_manager_loaded = Some(true);
                        sub_systems.config_manager =
                            self.loaded_config_manager.lock().unwrap().take().unwrap();

                        self.currently_loading =
                            "Configuration file loaded successfully".to_string();
                    }
                    Err(_error) => {
                        self.currently_loading =
                            "Error while loading Configuration file".to_string();
                    }
                },
                true => {}
            },
        }
        self
    }

    fn draw(&self, game_data: &mut GameData, canvas: &mut graphics::Canvas) {
        let loading_text = Text::new("Loading...".to_string());
        canvas.draw(
            &loading_text,
            graphics::DrawParam::from([200.0, 200.0]).color(Color::WHITE),
        );
        let currently_loading_text = Text::new(&self.currently_loading);
        canvas.draw(
            &currently_loading_text,
            graphics::DrawParam::from([200.0, 215.0]).color(Color::WHITE),
        )
    }
}
