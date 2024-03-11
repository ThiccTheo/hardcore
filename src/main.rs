mod game_state;
mod physics;
mod player;
mod sprite_flip;

use {
    bevy::prelude::*, game_state::GameState, physics::PhysicsPlugin, player::PlayerPlugin,
    sprite_flip::SpriteFlipPlugin,
};

fn main() {
    App::new()
        .init_state::<GameState>()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            SpriteFlipPlugin,
            PhysicsPlugin,
            PlayerPlugin,
        ))
        .run();
}
