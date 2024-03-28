use {
    super::{game_state::GameState, level},
    bevy::prelude::*,
};

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TileSpawnEvent>().add_systems(
            OnEnter(GameState::Playing),
            on_tile_spawn.after(level::spawn_entities),
        );
    }
}

#[derive(Event)]
pub struct TileSpawnEvent {
    pub pos: Vec2,
}

fn on_tile_spawn(mut cmds: Commands, mut tile_spawn_evr: EventReader<TileSpawnEvent>) {
    for TileSpawnEvent { pos: tile_pos } in tile_spawn_evr.read() {
        cmds.spawn(
            (SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::splat(16.)),
                    ..default()
                },
                transform: Transform::from_translation(tile_pos.extend(1.)),
                ..default()
            }),
        );
    }
}
