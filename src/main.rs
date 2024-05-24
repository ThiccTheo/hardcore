mod animation;
mod tile;
mod game_state;
mod mouse_position;
mod physics;
mod player;
mod sprite_flip;
mod level;

use {
    animation::AnimationPlugin,
    bevy::prelude::*,
    bevy_ecs_tilemap::TilemapPlugin,
    bevy_rapier2d::prelude::*,
    tile::TilePlugin,
    game_state::GameState,
    leafwing_input_manager::prelude::*,
    mouse_position::MousePositionPlugin,
    physics::PhysicsPlugin,
    player::{PlayerAction, PlayerPlugin},
    sprite_flip::SpriteFlipPlugin,
    level::LevelPlugin,
};

fn main() {
    App::new()
        .init_state::<GameState>()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            RapierPhysicsPlugin::<NoUserData>::default(),
            // RapierDebugRenderPlugin::default(),
            InputManagerPlugin::<PlayerAction>::default(),
            TilemapPlugin,
            MousePositionPlugin,
            SpriteFlipPlugin,
            AnimationPlugin,
            PhysicsPlugin,
            LevelPlugin,
            PlayerPlugin,
            TilePlugin,
        ))
        .run();
}
