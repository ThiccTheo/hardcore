mod animation;
mod game_state;
mod mouse_position;
mod physics;
mod player;
mod sprite_flip;

use {
    animation::AnimationPlugin,
    bevy::prelude::*,
    bevy_rapier2d::prelude::*,
    game_state::GameState,
    leafwing_input_manager::prelude::*,
    mouse_position::MousePositionPlugin,
    physics::PhysicsPlugin,
    player::{PlayerAction, PlayerPlugin},
    sprite_flip::SpriteFlipPlugin,
};

fn main() {
    App::new()
        .init_state::<GameState>()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            InputManagerPlugin::<PlayerAction>::default(),
            MousePositionPlugin,
            SpriteFlipPlugin,
            AnimationPlugin,
            PhysicsPlugin,
            PlayerPlugin,
        ))
        .run();
}
