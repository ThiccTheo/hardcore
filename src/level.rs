use {
    super::{game_state::GameState, tile::TileSpawnEvent},
    bevy::prelude::*,
    bitflags::bitflags,
    rand::Rng,
};

pub const LEVEL_SIZE: Vec2 = Vec2::splat(32.);
const SECTOR_ROWS: usize = 4;
const SECTOR_COLS: usize = 4;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            generate_sector_layout
                .pipe(generate_level_layout)
                .pipe(signal_entity_spawns),
        );
    }
}

type LevelLayout = [[[[u8; LEVEL_SIZE.x as usize / SECTOR_COLS];
    LEVEL_SIZE.y as usize / SECTOR_ROWS]; SECTOR_COLS]; SECTOR_ROWS];

type SectorLayout = Vec<Vec<SectorType>>;

bitflags! {
    #[rustfmt::skip]
    #[derive(Clone, Debug, PartialEq, Eq)]
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
    let mut level_layout = vec![vec![SectorType::CLOSED; SECTOR_COLS]; SECTOR_ROWS];

    let entrance_sector = rand::thread_rng().gen_range(0..SECTOR_COLS);
    level_layout[0][entrance_sector] |= SectorType::ENTRANCE;

    let exit_sector = rand::thread_rng().gen_range(0..SECTOR_COLS);
    level_layout[SECTOR_ROWS - 1][exit_sector] |= SectorType::EXIT;

    let mut down_sectors = vec![0; SECTOR_ROWS];
    let mut up_sectors = vec![0; SECTOR_ROWS];

    for y in 0..(SECTOR_ROWS - 1) {
        down_sectors[y] = rand::thread_rng().gen_range(0..SECTOR_COLS);
        level_layout[y][down_sectors[y]] |= SectorType::OPEN_DOWN;

        up_sectors[y + 1] = down_sectors[y];
        level_layout[y + 1][up_sectors[y + 1]] |= SectorType::OPEN_UP;
    }

    let make_inclusive_range = |a: usize, b: usize| {
        if a < b {
            Some(a..=b)
        } else if b < a {
            Some(b..=a)
        } else {
            None
        }
    };

    for y in 0..SECTOR_ROWS {
        let connected_sectors = if y == 0 {
            make_inclusive_range(entrance_sector, down_sectors[y])
        } else if (1..SECTOR_ROWS - 1).contains(&y) {
            make_inclusive_range(up_sectors[y], down_sectors[y])
        } else {
            make_inclusive_range(exit_sector, up_sectors[y])
        };
        let Some(connected_sectors) = connected_sectors else {
            continue;
        };

        level_layout[y][*connected_sectors.start()] |= SectorType::OPEN_RIGHT;
        level_layout[y][*connected_sectors.end()] |= SectorType::OPEN_LEFT;

        for x in *connected_sectors.start() + 1..*connected_sectors.end() {
            level_layout[y][x] |= SectorType::OPEN_LEFT | SectorType::OPEN_RIGHT;
        }
    }
    level_layout
}

pub fn generate_level_layout(In(sector_layout): In<SectorLayout>) -> LevelLayout {
    let mut level_layout = LevelLayout::default();

    for y in 0..SECTOR_ROWS {
        for x in 0..SECTOR_COLS {
            let sector = &sector_layout[y][x];
            let mut contents = [
                [1, 1, 1, 1, 1, 1, 1, 1],
                [1, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1, 1, 1],
            ];
            if sector.intersects(SectorType::OPEN_UP) {
                contents[0] = [1, 0, 0, 0, 0, 0, 0, 1];
            }
            if sector.intersects(SectorType::OPEN_DOWN) {
                contents[contents.len() - 1] = [1, 0, 0, 0, 0, 0, 0, 1];
            }
            if sector.intersects(SectorType::OPEN_LEFT) {
                for i in 1..SECTOR_ROWS - 1 {
                    contents[i][0] = 0;
                }
            }
            if sector.intersects(SectorType::OPEN_RIGHT) {
                for i in 1..SECTOR_ROWS - 1 {
                    contents[i][contents.len() - 1] = 0;
                }
            }
            // something is wrong here
            if sector.intersects(SectorType::ENTRANCE) {
                contents[contents.len() - 2][3] = 2;
            } else if sector.intersects(SectorType::EXIT) {
                contents[contents.len() - 2][3] = 3;
            }
            level_layout[y][x] = contents;
        }
    }
    level_layout
}

fn signal_entity_spawns(
    In(level_layout): In<LevelLayout>,
    mut tile_spawn_evw: EventWriter<TileSpawnEvent>,
) {
    for r in 0..SECTOR_ROWS {
        for c in 0..SECTOR_COLS {
            for y in 0..LEVEL_SIZE.y as usize / SECTOR_ROWS {
                for x in 0..LEVEL_SIZE.x as usize / SECTOR_COLS {
                    if level_layout[r][c][y][x] != 0 {
                        tile_spawn_evw.send(TileSpawnEvent {
                            pos: Vec2::new(
                                (x + (c * LEVEL_SIZE.x as usize / SECTOR_COLS)) as f32,
                                (y + (r * LEVEL_SIZE.y as usize / SECTOR_ROWS)) as f32,
                            ),
                            tex_idx: match level_layout[r][c][y][x] {
                                1 => 0,
                                2 => 2,
                                3 => 1,
                                _ => 0,
                            },
                        });
                    }
                }
            }
        }
    }
}
