use {
    super::game_state::GameState,
    bevy::prelude::*,
    bitflags::bitflags,
    rand::Rng,
};

pub const LEVEL_SIZE: Vec2 = Vec2::splat(64.);

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelLayout::new()).
        add_systems(
            OnEnter(GameState::Playing),
            generate_level_layout.pipe(convert_sectors_to_level),
        );
    }
}

#[derive(Resource, Deref, DerefMut)]
struct LevelLayout(Vec<Vec<u8>>);

impl LevelLayout {
    fn new() -> Self {
        Self(vec![vec![0; 64]; 64])
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

type SectorLayout = Vec<Vec<SectorType>>;

fn generate_level_layout() -> SectorLayout {
    let rows = 4;
    let cols = 4;
    let mut level_layout = vec![vec![SectorType::CLOSED; cols]; rows];

    let entrance_sector = rand::thread_rng().gen_range(0..cols);
    level_layout[0][entrance_sector] |= SectorType::ENTRANCE;

    let exit_sector = rand::thread_rng().gen_range(0..cols);
    level_layout[rows - 1][exit_sector] |= SectorType::EXIT;

    let mut down_sectors = vec![0; rows];
    let mut up_sectors = vec![0; rows];

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

pub fn convert_sectors_to_level(
    In(sector_layout): In<SectorLayout>,
    mut level_layout: ResMut<LevelLayout>,
) {
    for (sector_x, sector_y, sector_type) in sector_layout.iter().enumerate().flat_map(|(sector_y, sectors)| {
        sectors.iter()
            .enumerate()
            .map(move |(sector_x, sector_type)| (sector_x, sector_y, sector_type))
    }) {

    }
}

fn spawn_entities() {
    
}