use {
    super::{
        game_state::{GameState, PlayingEntity},
        level,
        asset_owners::TextureAtlasOwner,
        tile::{Tile, TILE_SIZE, TILE_Z},
    },
    bevy::prelude::*,
    bevy_rapier2d::prelude::*,
    bevy_tnua::TnuaGhostPlatform,
};

const DOOR_Z: f32 = TILE_Z;
const DOOR_PLATFORM_SIZE: Vec2 = Vec2::new(TILE_SIZE.x, TILE_SIZE.y / 6.);

#[derive(Component, PartialEq, Eq, Clone, Copy)]
pub enum Door {
    Entrance,
    Exit,
}

#[derive(Event)]
pub struct DoorSpawnEvent {
    pub pos: Vec2,
    pub tex_idx: usize,
    pub is_exit: bool,
}

fn on_door_spawn(
    mut door_spawn_evr: EventReader<DoorSpawnEvent>,
    mut cmds: Commands,
    tile_assets: Res<TextureAtlasOwner<Tile>>,
) {
    for &DoorSpawnEvent {
        pos,
        tex_idx,
        is_exit,
    } in door_spawn_evr.read()
    {
        cmds.spawn((
            if is_exit { Door::Exit } else { Door::Entrance },
            PlayingEntity,
            SpriteSheetBundle {
                transform: Transform::from_translation(pos.extend(DOOR_Z)),
                texture: tile_assets.tex.clone_weak(),
                atlas: TextureAtlas {
                    layout: tile_assets.layout.clone_weak(),
                    index: tex_idx,
                },
                ..default()
            },
            Collider::cuboid(TILE_SIZE.x / 2., TILE_SIZE.y / 2.),
            Sensor,
        ));

        cmds.spawn((
            PlayingEntity,
            TnuaGhostPlatform,
            Collider::cuboid(DOOR_PLATFORM_SIZE.x / 2., DOOR_PLATFORM_SIZE.y / 2.),
            SpriteSheetBundle {
                sprite: Sprite {
                    custom_size: Some(DOOR_PLATFORM_SIZE),
                    ..default()
                },
                transform: Transform::from_translation(
                    (pos - Vec2::Y * (TILE_SIZE.y / 2. + DOOR_PLATFORM_SIZE.y / 2.))
                        .extend(DOOR_Z - 0.5),
                ),
                ..default()
            },
            SolverGroups {
                memberships: Group::empty(),
                filters: Group::empty(),
            },
        ));
    }
}

pub fn door_plugin(app: &mut App) {
    app.add_event::<DoorSpawnEvent>().add_systems(
        OnEnter(GameState::Playing),
        (on_door_spawn.after(level::signal_level_object_spawns)).chain(),
    );
}
