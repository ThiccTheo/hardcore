use {
    super::{
        game_state::GameState,
        player::PlayerSpawnEvent,
        spike::SpikeSpawnEvent,
        tile::{TileSpawnEvent, TILE_SIZE},
    },
    bevy::prelude::*,
    bitflags::bitflags,
    rand::Rng,
    static_assertions::const_assert,
    std::cmp::Ordering,
};

const SECTOR_COLS: usize = 4;
const SECTOR_ROWS: usize = 4;
const SECTOR_SIZE: Vec2 = Vec2::new(8., 9.);
pub const LEVEL_SIZE: Vec2 = Vec2::new(
    SECTOR_SIZE.x * SECTOR_COLS as f32,
    SECTOR_SIZE.y * SECTOR_ROWS as f32,
);

const_assert!(SECTOR_COLS >= 1 && SECTOR_ROWS >= 2);
const_assert!(SECTOR_SIZE.x >= 4. && SECTOR_SIZE.y >= 4. && SECTOR_SIZE.y <= 9.);

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum LevelObject {
    #[default]
    Background,
    Entrance,
    Spike,
    Tile,
    Exit,
    Path,
}

type SectorLayout = [[SectorType; SECTOR_COLS]; SECTOR_ROWS];

type LevelLayout =
    [[[[LevelObject; SECTOR_SIZE.x as usize]; SECTOR_SIZE.y as usize]; SECTOR_COLS]; SECTOR_ROWS];

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

#[derive(Resource)]
pub struct LevelData {
    world: u8,
    level: u8,
}

impl LevelData {
    fn update(&mut self) {
        if self.level == 1 {
            self.world += 1;
            self.level = 0;
        }
        self.level += 1;
    }
}

impl Default for LevelData {
    fn default() -> Self {
        Self { world: 1, level: 1 }
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

    for r in 0..SECTOR_ROWS {
        for c in 0..SECTOR_COLS {
            let sector_type = &sector_layout[r][c];
            let mut sector_contents =
                [[LevelObject::Background; SECTOR_SIZE.x as usize]; SECTOR_SIZE.y as usize];
            sector_contents[0] = [LevelObject::Tile; SECTOR_SIZE.x as usize];
            sector_contents[SECTOR_SIZE.y as usize - 1] =
                [LevelObject::Tile; SECTOR_SIZE.x as usize];
            for i in 1..SECTOR_SIZE.y as usize - 1 {
                sector_contents[i][0] = LevelObject::Tile;
                sector_contents[i][SECTOR_SIZE.x as usize - 1] = LevelObject::Tile;
            }

            if sector_type.intersects(SectorType::OPEN_UP) {
                // sector_contents[0] = [LevelObject::Background; SECTOR_SIZE.x as usize];
                // sector_contents[0][0] = LevelObject::Tile;
                // sector_contents[0][SECTOR_SIZE.x as usize - 1] = LevelObject::Tile;
                for i in 0..=SECTOR_SIZE.y as usize / 2 {
                    sector_contents[i][SECTOR_SIZE.x as usize / 2] = LevelObject::Path;
                }
            }
            if sector_type.intersects(SectorType::OPEN_DOWN) {
                // sector_contents[SECTOR_SIZE.y as usize - 1] = [LevelObject::Background; SECTOR_SIZE.x as usize];
                // sector_contents[SECTOR_SIZE.y as usize - 1][0] = LevelObject::Tile;
                // sector_contents[SECTOR_SIZE.y as usize - 1][SECTOR_SIZE.x as usize - 1] = LevelObject::Tile;
                for i in SECTOR_SIZE.y as usize / 2..SECTOR_SIZE.y as usize {
                    sector_contents[i][SECTOR_SIZE.x as usize / 2] = LevelObject::Path;
                }
            }
            if sector_type.intersects(SectorType::OPEN_LEFT) {
                // for i in 1..SECTOR_SIZE.y as usize - 1 {
                //     sector_contents[i][0] = LevelObject::Background;
                // }
                for i in 0..=SECTOR_SIZE.x as usize / 2 {
                    sector_contents[SECTOR_SIZE.y as usize / 2][i] = LevelObject::Path;
                }
            }
            if sector_type.intersects(SectorType::OPEN_RIGHT) {
                // for i in 1..SECTOR_SIZE.y as usize - 1 {
                //     sector_contents[i][SECTOR_SIZE.x as usize - 1] = LevelObject::Background;
                // }
                for i in SECTOR_SIZE.x as usize / 2..SECTOR_SIZE.x as usize {
                    sector_contents[SECTOR_SIZE.y as usize / 2][i] = LevelObject::Path;
                }
            }
            if sector_type.intersects(SectorType::ENTRANCE) {
                sector_contents[SECTOR_SIZE.y as usize / 2][SECTOR_SIZE.x as usize / 2] =
                    LevelObject::Entrance;
            } else if sector_type.intersects(SectorType::EXIT) {
                sector_contents[SECTOR_SIZE.y as usize / 2][SECTOR_SIZE.x as usize / 2] =
                    LevelObject::Exit;
            }

            // STILL IN TEST PHASE
            for y in 1..SECTOR_SIZE.y as usize - 1 {
                for x in 1..SECTOR_SIZE.x as usize - 1 {
                    if sector_contents[y][x] == LevelObject::Background
                        && [
                            sector_contents[y - 1][x - 1],
                            sector_contents[y - 1][x],
                            sector_contents[y - 1][x + 1],
                            sector_contents[y][x - 1],
                            sector_contents[y][x + 1],
                            sector_contents[y + 1][x - 1],
                            sector_contents[y + 1][x],
                            sector_contents[y + 1][x + 1],
                        ]
                        .into_iter()
                        .any(|neighbor| neighbor == LevelObject::Tile)
                        && rand::thread_rng().gen_ratio(1, 3)
                    {
                        sector_contents[y][x] = LevelObject::Tile;
                    }
                }
            }
            for y in 0..SECTOR_SIZE.y as usize - 1 {
                for x in 0..SECTOR_SIZE.x as usize {
                    if sector_contents[y][x] == LevelObject::Background
                        && sector_contents[y + 1][x] == LevelObject::Tile
                        && rand::thread_rng().gen_ratio(1, 4)
                    {
                        sector_contents[y][x] = LevelObject::Spike;
                    }
                }
            }
            level_layout[r][c] = sector_contents;
        }
    }
    level_layout
}

pub fn signal_level_object_spawns(
    In(level_layout): In<LevelLayout>,
    level_data: Res<LevelData>,
    mut tile_spawn_evw: EventWriter<TileSpawnEvent>,
    mut player_spawn_evw: EventWriter<PlayerSpawnEvent>,
    mut spike_spawn_evw: EventWriter<SpikeSpawnEvent>,
) {
    for r in 0..SECTOR_ROWS {
        for c in 0..SECTOR_COLS {
            for y in 0..SECTOR_SIZE.y as usize {
                for x in 0..SECTOR_SIZE.x as usize {
                    let pos = (Transform::from_translation(
                        (-Vec2::new(LEVEL_SIZE.x - 1., LEVEL_SIZE.y - 1.) * TILE_SIZE / 2.)
                            .extend(default()),
                    ) * Transform::from_translation(
                        (Vec2::new(
                            (x + c * SECTOR_SIZE.x as usize) as f32,
                            LEVEL_SIZE.y - (y + r * SECTOR_SIZE.y as usize) as f32 - 1.,
                        ) * TILE_SIZE)
                            .extend(default()),
                    ))
                    .translation
                    .truncate();

                    match level_layout[r][c][y][x] {
                        LevelObject::Tile => {
                            tile_spawn_evw.send(TileSpawnEvent {
                                pos,
                                tex_idx: 5 + level_data.world as usize,
                                has_collider: true,
                                is_door: false,
                            });
                        }
                        LevelObject::Entrance => {
                            player_spawn_evw.send(PlayerSpawnEvent { pos });
                            tile_spawn_evw.send(TileSpawnEvent {
                                pos,
                                tex_idx: 75 + level_data.world as usize,
                                has_collider: false,
                                is_door: true,
                            });
                        }
                        LevelObject::Exit => {
                            tile_spawn_evw.send(TileSpawnEvent {
                                pos,
                                tex_idx: 75,
                                has_collider: false,
                                is_door: true,
                            });
                        }
                        LevelObject::Spike => {
                            spike_spawn_evw.send(SpikeSpawnEvent { pos });
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}

pub fn level_plugin(app: &mut App) {
    app.insert_resource(LevelData::default()).add_systems(
        OnEnter(GameState::Playing),
        generate_sector_layout
            .pipe(generate_level_layout)
            .pipe(signal_level_object_spawns),
    );
}
