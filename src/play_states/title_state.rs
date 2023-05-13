use crate::{GameData, SubSystems};

use super::play_state::PlayState;

pub struct TitleState {}

impl PlayState for TitleState {
    fn update(
        self: Box<Self>,
        sub_systems: &mut SubSystems,
        ctx: &mut ggez::Context,
    ) -> Box<dyn PlayState> {
        todo!()
    }

    fn draw(&self, game_data: &mut GameData, canvas: &mut ggez::graphics::Canvas) {
        todo!()
    }
}
