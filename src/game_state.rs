use bevy::prelude::*;

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum GameState {
    #[default]
    Playing,
    Transition,
}

#[derive(Component)]
pub struct PlayingEntity;

#[derive(Component)]
struct TransitionEntity;

fn destroy_state<T: Component>(mut cmds: Commands, game_state_qry: Query<Entity, With<T>>) {
    for id in &game_state_qry {
        cmds.entity(id).despawn_recursive();
    }
}

pub fn game_state_plugin(app: &mut App) {
    app.add_systems(
        OnTransition {
            from: GameState::Playing,
            to: GameState::Transition,
        },
        destroy_state::<PlayingEntity>,
    )
    .add_systems(
        OnTransition {
            from: GameState::Transition,
            to: GameState::Playing,
        },
        destroy_state::<TransitionEntity>,
    )
    // temporary
    .add_systems(
        OnEnter(GameState::Transition),
        |mut next_state: ResMut<NextState<GameState>>| next_state.set(GameState::Playing),
    );
}
