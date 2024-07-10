use {
    super::{
        asset_owners::TextureAtlasOwner,
        level,
        tile::{Tile, TILE_SIZE, TILE_Z},
    },
    crate::GameState,
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
            StateScoped(GameState::Playing),
            SpriteBundle {
                transform: Transform::from_translation(pos.extend(DOOR_Z)),
                texture: tile_assets.texture(),
                ..default()
            },
            TextureAtlas {
                layout: tile_assets.layout(),
                index: tex_idx,
            },
            Collider::cuboid(TILE_SIZE.x / 2., TILE_SIZE.y / 2.),
            Sensor,
        ));

        cmds.spawn((
            StateScoped(GameState::Playing),
            TnuaGhostPlatform,
            Collider::cuboid(DOOR_PLATFORM_SIZE.x / 2., DOOR_PLATFORM_SIZE.y / 2.),
            SpriteBundle {
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
