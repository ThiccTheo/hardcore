mod animation;
mod game_state;
mod physics;
mod player;
mod sprite_flip;

use {
    animation::AnimationPlugin, bevy::prelude::*, bevy_rapier2d::prelude::*, game_state::GameState,
    physics::PhysicsPlugin, player::PlayerPlugin, sprite_flip::SpriteFlipPlugin,
};

fn main() {
    App::new()
        .init_state::<GameState>()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            SpriteFlipPlugin,
            AnimationPlugin,
            PhysicsPlugin,
            PlayerPlugin,
        ))
        .run();
}
