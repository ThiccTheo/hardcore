mod animation;
mod game_state;
mod level;
mod main_camera;
mod mouse_position;
mod physics;
mod player;
mod sprite_flip;
mod tile;

use {
    animation::AnimationPlugin,
    bevy::prelude::*,
    bevy_ecs_tilemap::TilemapPlugin,
    bevy_rapier2d::prelude::*,
    game_state::GameState,
    leafwing_input_manager::prelude::*,
    level::LevelPlugin,
    main_camera::MainCameraPlugin,
    mouse_position::MousePositionPlugin,
    physics::PhysicsPlugin,
    player::{PlayerAction, PlayerPlugin},
    sprite_flip::SpriteFlipPlugin,
    tile::TilePlugin,
};

fn main() {
    App::new()
        .init_state::<GameState>()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            RapierPhysicsPlugin::<NoUserData>::default(),
            //RapierDebugRenderPlugin::default(),
            InputManagerPlugin::<PlayerAction>::default(),
            TilemapPlugin,
            MousePositionPlugin,
            SpriteFlipPlugin,
            AnimationPlugin,
            PhysicsPlugin,
            MainCameraPlugin,
            LevelPlugin,
            PlayerPlugin,
            TilePlugin,
        ))
        .run();
}
