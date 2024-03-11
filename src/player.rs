use {super::game_state::GameState, bevy::prelude::*};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut cmds: Commands) {
    cmds.spawn((Player, SpriteBundle::default()))
        .with_children(|parent| {
            parent.spawn(Camera2dBundle::default());
        });
}
