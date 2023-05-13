use ggez::{graphics, Context};

use crate::{GameData, SubSystems};

pub trait PlayState {
    fn update(
        self: Box<Self>,
        sub_systems: &mut SubSystems,
        ctx: &mut Context,
    ) -> Box<dyn PlayState>;
    fn draw(&self, game_data: &mut GameData, canvas: &mut graphics::Canvas);
}

// impl dyn PlayState {}
