use {
    super::{
        game_state::GameState,
        player::{PlayerSpawnEvent, PLAYER_ID},
        // skeleton::{SkeletonSpawnEvent, SKELETON_ID},
        // slime::{SlimeSpawnEvent, SLIME_ID},
        tile::{TileSpawnEvent, TILE_ID, TILE_SIZE},
    },
    bevy::prelude::*,
    bevy_ecs_tilemap::prelude::*,
    bitflags::bitflags,
    rand::Rng,
    std::cmp::Ordering,
};

const BG_ID: u8 = 0;
const EXIT_ID: u8 = u8::MAX;
const SECTOR_COLS: usize = 4;
const SECTOR_ROWS: usize = 4;
const SECTOR_SIZE: Vec2 = Vec2::new(16., 8.);
pub const LEVEL_SIZE: Vec2 = Vec2::new(
    SECTOR_SIZE.x * SECTOR_COLS as f32,
    SECTOR_SIZE.y * SECTOR_ROWS as f32,
);

type LevelLayout =
    [[[[u8; SECTOR_SIZE.x as usize]; SECTOR_SIZE.y as usize]; SECTOR_COLS]; SECTOR_ROWS];

type SectorLayout = [[SectorType; SECTOR_COLS]; SECTOR_ROWS];

bitflags! {
    #[rustfmt::skip]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct SectorType: u8 {
        const ENTRANCE   = 0b00100000;
        const EXIT       = 0b00010000;
        const OPEN_UP    = 0b00001000;
        const OPEN_DOWN  = 0b00000100;
        const OPEN_LEFT  = 0b00000010;
        const OPEN_RIGHT = 0b00000001;
        const CLOSED     = 0b00000000;
    }
}

fn generate_sector_layout() -> SectorLayout {
    let mut sector_layout = [[SectorType::CLOSED; SECTOR_COLS]; SECTOR_ROWS];

    let entrance_pos = rand::thread_rng().gen_range(0..SECTOR_COLS);
    sector_layout[0][entrance_pos] |= SectorType::ENTRANCE;

    let exit_pos = rand::thread_rng().gen_range(0..SECTOR_COLS);
    sector_layout[SECTOR_ROWS - 1][exit_pos] |= SectorType::EXIT;

    let mut down_sectors = [0; SECTOR_ROWS];
    let mut up_sectors = [0; SECTOR_ROWS];

    for y in 0..SECTOR_ROWS - 1 {
        down_sectors[y] = rand::thread_rng().gen_range(0..SECTOR_COLS);
        sector_layout[y][down_sectors[y]] |= SectorType::OPEN_DOWN;

        up_sectors[y + 1] = down_sectors[y];
        sector_layout[y + 1][up_sectors[y + 1]] |= SectorType::OPEN_UP;
    }

    let make_inclusive_range = |a: usize, b: usize| match a.cmp(&b) {
        Ordering::Less => Some(a..=b),
        Ordering::Greater => Some(b..=a),
        Ordering::Equal => None,
    };

    for y in 0..SECTOR_ROWS {
        let connected_sectors = if y == 0 {
            make_inclusive_range(entrance_pos, down_sectors[y])
        } else if (1..SECTOR_ROWS - 1).contains(&y) {
            make_inclusive_range(up_sectors[y], down_sectors[y])
        } else {
            make_inclusive_range(exit_pos, up_sectors[y])
        };
        let Some(connected_sectors) = connected_sectors else {
            continue;
        };

        sector_layout[y][*connected_sectors.start()] |= SectorType::OPEN_RIGHT;
        sector_layout[y][*connected_sectors.end()] |= SectorType::OPEN_LEFT;

        for x in *connected_sectors.start() + 1..*connected_sectors.end() {
            sector_layout[y][x] |= SectorType::OPEN_LEFT | SectorType::OPEN_RIGHT;
        }
    }
    sector_layout
}

fn generate_level_layout(In(sector_layout): In<SectorLayout>) -> LevelLayout {
    let mut level_layout = LevelLayout::default();

    for y in 0..SECTOR_ROWS {
        for x in 0..SECTOR_COLS {
            let sector_type = &sector_layout[y][x];
            let mut sector_contents = [[BG_ID; SECTOR_SIZE.x as usize]; SECTOR_SIZE.y as usize];
            sector_contents[0] = [TILE_ID; SECTOR_SIZE.x as usize];
            sector_contents[SECTOR_SIZE.y as usize - 1] = [TILE_ID; SECTOR_SIZE.x as usize];
            for i in 1..SECTOR_SIZE.y as usize - 1 {
                sector_contents[i][0] = TILE_ID;
                sector_contents[i][SECTOR_SIZE.x as usize - 1] = TILE_ID;
            }

            if sector_type.intersects(SectorType::OPEN_UP) {
                sector_contents[0] = [BG_ID; SECTOR_SIZE.x as usize];
                sector_contents[0][0] = TILE_ID;
                sector_contents[0][SECTOR_SIZE.x as usize - 1] = TILE_ID;
            }
            if sector_type.intersects(SectorType::OPEN_DOWN) {
                sector_contents[SECTOR_SIZE.y as usize - 1] = [BG_ID; SECTOR_SIZE.x as usize];
                sector_contents[SECTOR_SIZE.y as usize - 1][0] = TILE_ID;
                sector_contents[SECTOR_SIZE.y as usize - 1][SECTOR_SIZE.x as usize - 1] = TILE_ID;
            }
            if sector_type.intersects(SectorType::OPEN_LEFT) {
                for i in 1..SECTOR_SIZE.y as usize - 1 {
                    sector_contents[i][0] = BG_ID;
                }
            }
            if sector_type.intersects(SectorType::OPEN_RIGHT) {
                for i in 1..SECTOR_SIZE.y as usize - 1 {
                    sector_contents[i][SECTOR_SIZE.x as usize - 1] = BG_ID;
                }
            }
            if sector_type.intersects(SectorType::ENTRANCE) {
                sector_contents[SECTOR_SIZE.y as usize - 2][SECTOR_SIZE.x as usize / 2] = PLAYER_ID;
            } else if sector_type.intersects(SectorType::EXIT) {
                sector_contents[SECTOR_SIZE.y as usize - 2][SECTOR_SIZE.x as usize / 2] = EXIT_ID;
            }
            level_layout[y][x] = sector_contents;
        }
    }
    level_layout
}

pub fn signal_entity_spawns(
    In(level_layout): In<LevelLayout>,
    tilemap_qry: Query<&Transform, With<TileStorage>>,
    mut tile_spawn_evw: EventWriter<TileSpawnEvent>,
    mut player_spawn_evw: EventWriter<PlayerSpawnEvent>,
    // mut skeleton_spawn_evw: EventWriter<SkeletonSpawnEvent>,
    // mut slime_spawn_evw: EventWriter<SlimeSpawnEvent>,
) {
    let tilemap_xform = tilemap_qry.single();

    for r in 0..SECTOR_ROWS {
        for c in 0..SECTOR_COLS {
            for y in 0..SECTOR_SIZE.y as usize {
                for x in 0..SECTOR_SIZE.x as usize {
                    let entity_type = level_layout[r][c][y][x];
                    let tile_pos = TilePos::new(
                        (x + c * SECTOR_SIZE.x as usize) as u32,
                        LEVEL_SIZE.y as u32 - (y + r * SECTOR_SIZE.y as usize) as u32 - 1,
                    );
                    let world_pos = (*tilemap_xform
                        * Transform::from_translation(
                            tile_pos
                                .center_in_world(&TILE_SIZE.into(), &TilemapType::Square)
                                .extend(default()),
                        ))
                    .translation
                    .truncate();

                    match entity_type {
                        TILE_ID => {
                            tile_spawn_evw.send(TileSpawnEvent {
                                tile_pos,
                                world_pos,
                                tex_idx: TileTextureIndex(10),
                            });
                        }
                        PLAYER_ID => {
                            player_spawn_evw.send(PlayerSpawnEvent { pos: world_pos });
                            tile_spawn_evw.send(TileSpawnEvent {
                                tile_pos,
                                world_pos,
                                tex_idx: TileTextureIndex(1),
                            });
                        }
                        EXIT_ID => {
                            tile_spawn_evw.send(TileSpawnEvent {
                                tile_pos,
                                world_pos,
                                tex_idx: TileTextureIndex(2),
                            });
                        }
                        // SKELETON_ID => {
                        //     skeleton_spawn_evw.send(SkeletonSpawnEvent { pos: world_pos });
                        // }
                        // SLIME_ID => {
                        //     slime_spawn_evw.send(SlimeSpawnEvent { pos: world_pos });
                        // }
                        _ => (),
                    }
                }
            }
        }
    }
}

pub fn level_plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Playing),
        generate_sector_layout
            .pipe(generate_level_layout)
            .pipe(signal_entity_spawns),
    );
}
