use {
    super::{game_state::GameState, level, texture_atlas_owner::TextureAtlasOwner},
    bevy::prelude::*,
    bevy_rapier2d::prelude::*,
    bevy_tnua::TnuaGhostPlatform,
};

const TILE_Z: f32 = 1.;
pub const TILE_SIZE: Vec2 = Vec2::splat(128.);
const DOOR_PLATFORM_SIZE: Vec2 = Vec2::new(TILE_SIZE.x, TILE_SIZE.y / 6.);

#[derive(Component)]
pub struct Tile;

#[derive(Event)]
pub struct TileSpawnEvent {
    pub pos: Vec2,
    pub tex_idx: usize,
    pub has_collider: bool,
    pub is_door: bool,
}

fn on_tile_spawn(
    mut tile_spawn_evr: EventReader<TileSpawnEvent>,
    mut cmds: Commands,
    tile_assets: Res<TextureAtlasOwner<Tile>>,
) {
    for &TileSpawnEvent {
        pos,
        tex_idx,
        has_collider,
        is_door,
    } in tile_spawn_evr.read()
    {
        let tile_id = cmds
            .spawn((
                Tile,
                SpriteSheetBundle {
                    transform: Transform::from_translation(pos.extend(TILE_Z)),
                    texture: tile_assets.tex.clone_weak(),
                    atlas: TextureAtlas {
                        layout: tile_assets.layout.clone_weak(),
                        index: tex_idx,
                    },
                    ..default()
                },
            ))
            .id();

        if has_collider {
            cmds.entity(tile_id)
                .insert(Collider::cuboid(TILE_SIZE.x / 2., TILE_SIZE.y / 2.));
        }
        if is_door {
            cmds.spawn((
                TnuaGhostPlatform,
                Collider::cuboid(DOOR_PLATFORM_SIZE.x / 2., DOOR_PLATFORM_SIZE.y / 2.),
                SpriteSheetBundle {
                    sprite: Sprite {
                        custom_size: Some(DOOR_PLATFORM_SIZE),
                        ..default()
                    },
                    transform: Transform::from_translation(
                        (pos - Vec2::Y * (TILE_SIZE.y / 2. + DOOR_PLATFORM_SIZE.y / 2.))
                            .extend(TILE_Z - 0.5),
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
}

pub fn tile_plugin(app: &mut App) {
    app.add_event::<TileSpawnEvent>()
        .add_systems(
            Startup,
            |mut cmds: Commands,
             asset_server: Res<AssetServer>,
             mut tex_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>| {
                cmds.insert_resource(TextureAtlasOwner::<Tile>::new(
                    asset_server.load("tile.png"),
                    tex_atlas_layouts.add(TextureAtlasLayout::from_grid(
                        Vec2::splat(128.),
                        14,
                        7,
                        None,
                        None,
                    )),
                ));
            },
        )
        .add_systems(
            OnEnter(GameState::Playing),
            (on_tile_spawn.after(level::signal_level_object_spawns),).chain(),
        );
}
