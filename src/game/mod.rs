use bevy::prelude::*;
use player::PlayerPlugin;

mod player;

pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(PlayerPlugin);
	}
}
