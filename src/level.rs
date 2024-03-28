use {
    super::{game_state::GameState, player::PlayerSpawnEvent, tile::TileSpawnEvent},
    bevy::prelude::*,
    bitflags::bitflags,
    rand::Rng,
};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            generate_level_layout.pipe(spawn_entities),
        );
    }
}

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

fn generate_level_layout() -> Vec<Vec<SectorType>> {
    let rows = 4;
    let cols = 4;
    let mut level_layout = vec![vec![SectorType::CLOSED; cols]; rows];

    let entrance_sector = rand::thread_rng().gen_range(0..cols);
    level_layout[0][entrance_sector] |= SectorType::ENTRANCE;

    let exit_sector = rand::thread_rng().gen_range(0..cols);
    level_layout[rows - 1][exit_sector] |= SectorType::EXIT;

    let mut down_sectors = vec![0; rows]; // [0, rows)
    let mut up_sectors = vec![0; rows]; // (0, rows]

    for y in 0..(rows - 1) {
        down_sectors[y] = rand::thread_rng().gen_range(0..cols);
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

    for y in 0..rows {
        let connected_sectors = if y == 0 {
            make_inclusive_range(entrance_sector, down_sectors[y])
        } else if (1..rows - 1).contains(&y) {
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

pub fn spawn_entities(
    In(level_layout): In<Vec<Vec<SectorType>>>,
    mut player_spawn_evw: EventWriter<PlayerSpawnEvent>,
    mut tile_spawn_evw: EventWriter<TileSpawnEvent>
) {
    let (h, w) = (
        level_layout.len(),
        if level_layout.len() > 0 {
            level_layout[0].len()
        } else {
            0
        },
    );
    for (sector_x, sector_y, sector_type) in level_layout.iter().enumerate().flat_map(|(sector_y, sectors)| {
        sectors.iter()
            .enumerate()
            .map(move |(sector_x, sector_type)| (sector_x, sector_y, sector_type))
    }) {
        let pos = Vec2::new(
            sector_x as f32 * 16. - w as f32 * 16. / 2.,
            sector_y as f32 * 16. - h as f32 * 16. / 2.,
        );
        tile_spawn_evw.send(TileSpawnEvent { pos });

        player_spawn_evw.send(PlayerSpawnEvent {
            pos: Vec2::splat(0.),
        });
    }
}
