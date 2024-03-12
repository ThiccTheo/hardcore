use {
    super::{
        game_state::GameState,
        physics::{Acceleration, Grounded, NetDirection, TerminalVelocity},
        sprite_flip::Flippable,
    },
    bevy::prelude::*,
    bevy_rapier2d::prelude::*,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut tex_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    cmds.spawn((
        Player,
        SpriteSheetBundle {
            texture: asset_server.load("player.png"),
            atlas: TextureAtlas {
                layout: tex_atlas_layouts.add(TextureAtlasLayout::from_grid(
                    Vec2::splat(16.),
                    4,
                    2,
                    None,
                    None,
                )),
                index: 0,
            },
            ..default()
        },
        Flippable::default(),
        KinematicCharacterController::default(),
        Collider::capsule_y(3., 4.),
        Friction::coefficient(3.),
        Velocity::zero(),
        TerminalVelocity(Vec2::new(50., 200.)),
        Acceleration(Vec2::new(300., 500.)),
        NetDirection { x: 0, y: -1 },
        Grounded::default(),
    ))
    .with_children(|parent| {
        let mut cam = Camera2dBundle::default();
        cam.projection.scale /= 4.;
        parent.spawn(cam);
    });

    cmds.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(300., 16.)),
                ..default()
            },
            transform: Transform::from_xyz(0., -30., 1.),
            ..default()
        },
        Collider::cuboid(150., 8.),
    ));
}