mod game_state;
mod player;

use {bevy::prelude::*, game_state::GameState, player::PlayerPlugin};

fn main() {
    App::new()
        .init_state::<GameState>()
        .add_plugins((DefaultPlugins, PlayerPlugin))
        .run();
}
