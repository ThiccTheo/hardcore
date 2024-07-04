use {
    super::{
        combat::Health,
        game_state::{GameState, PlayingEntity},
        player::{self, Player, PLAYER_MAX_HEALTH},
        texture_atlas_owner::TextureAtlasOwner,
        tile::{Tile, TILE_SIZE},
    },
    crate::RESOLUTION,
    bevy::prelude::*,
    std::cmp::Ordering,
};

#[derive(Component)]
struct Healthbar;

fn spawn_hud(mut cmds: Commands, tile_assets: Res<TextureAtlasOwner<Tile>>) {
    cmds.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(RESOLUTION.x),
                height: Val::Px(RESOLUTION.y),
                ..default()
            },
            ..default()
        },
        PlayingEntity,
    ))
    .with_children(|screen| {
        screen
            .spawn((
                Healthbar,
                NodeBundle {
                    style: Style {
                        width: Val::Percent(30.),
                        height: Val::Percent(10.),
                        justify_content: JustifyContent::SpaceEvenly,
                        ..default()
                    },
                    ..default()
                },
            ))
            .with_children(|healthbar| {
                for _ in 0..PLAYER_MAX_HEALTH.0 / 2 {
                    healthbar.spawn(AtlasImageBundle {
                        style: Style { ..default() },
                        image: UiImage::new(tile_assets.tex.clone_weak()),
                        texture_atlas: TextureAtlas {
                            layout: tile_assets.layout.clone_weak(),
                            index: 39,
                        },
                        ..default()
                    });
                }
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
        OnEnter(GameState::Playing),
        spawn_hud.after(player::on_player_spawn),
    )
    .add_systems(Update, update_hud.run_if(in_state(GameState::Playing)));
}
