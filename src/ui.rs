use {
    super::{
        asset_owners::{FontOwner, TextureAtlasOwner},
        combat::Health,
        level::LevelInfo,
        player::{self, Player, PLAYER_MAX_HEALTH},
        tile::Tile,
    },
    crate::GameState,
    crate::RESOLUTION,
    bevy::prelude::*,
    std::cmp::Ordering,
};

#[derive(Component)]
struct Ui;

#[derive(Component)]
struct Healthbar;

fn spawn_hud(
    mut cmds: Commands,
    tile_assets: Res<TextureAtlasOwner<Tile>>,
    ui_font: Res<FontOwner<Ui>>,
    level_info: Res<LevelInfo>,
) {
    cmds.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(RESOLUTION.x),
                height: Val::Px(RESOLUTION.y),
                ..default()
            },
            ..default()
        },
        StateScoped(GameState::Playing),
    ))
    .with_children(|screen| {
        screen
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(10.),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            })
            .with_children(|hud| {
                hud.spawn((
                    Healthbar,
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(30.),
                            height: Val::Percent(100.),
                            justify_content: JustifyContent::SpaceEvenly,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|healthbar| {
                    for _ in 0..PLAYER_MAX_HEALTH.0 / 2 {
                        healthbar.spawn((
                            ImageBundle {
                                image: UiImage::new(tile_assets.texture()),
                                style: Style {
                                    max_width: Val::Percent(
                                        100. / (PLAYER_MAX_HEALTH.0 / 2) as f32,
                                    ),
                                    max_height: Val::Percent(100.),
                                    ..default()
                                },
                                ..default()
                            },
                            TextureAtlas {
                                layout: tile_assets.layout(),
                                index: 39,
                            },
                        ));
                    }
                });
                hud.spawn(TextBundle::from_section(
                    format!(
                        "{world}-{level}",
                        world = level_info.world(),
                        level = level_info.level()
                    ),
                    TextStyle {
                        font: ui_font.font(),
                        font_size: 40.,
                        color: Color::BLACK,
                        ..default()
                    },
                ));
            });
    });
}

fn update_hud(
    healthbar_qry: Query<&Children, With<Healthbar>>,
    mut tex_atlas_qry: Query<&mut TextureAtlas>,
    player_qry: Query<&Health, With<Player>>,
) {
    let mut player_hp = player_qry.single().0;

    for &heart_id in healthbar_qry.single().iter() {
        let Ok(mut heart_tex_atlas) = tex_atlas_qry.get_mut(heart_id) else {
            continue;
        };
        heart_tex_atlas.index = match player_hp.cmp(&1) {
            Ordering::Less => 39,
            Ordering::Equal => 53,
            Ordering::Greater => 67,
        };
        player_hp -= 2;
    }
}

pub fn ui_plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Setup),
        |mut cmds: Commands, asset_server: Res<AssetServer>| {
            cmds.insert_resource(FontOwner::<Ui>::new(asset_server.load("font.ttf")));
        },
    )
    .add_systems(
        OnEnter(GameState::Playing),
        spawn_hud.after(player::on_player_spawn),
    )
    .add_systems(Update, update_hud.run_if(in_state(GameState::Playing)));
}
