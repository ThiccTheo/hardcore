use bevy::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (adjust_sprite_indices, animate_sprites).chain());
    }
}

#[derive(Component, PartialEq, Default)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub fn adjust_sprite_indices(
    mut animation_qry: Query<(&mut TextureAtlas, &AnimationIndices), Changed<AnimationIndices>>,
) {
    for (mut tex_atlas, animation_indices) in animation_qry.iter_mut() {
        tex_atlas.index = animation_indices.first;
    }
}

fn animate_sprites(
    time: Res<Time>,
    mut animation_qry: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (animation_indices, mut animation_timer, mut tex_atlas) in animation_qry.iter_mut() {
        animation_timer.tick(time.delta());
        if animation_timer.just_finished() {
            tex_atlas.index = animation_indices.first
                + (tex_atlas.index + 1 - animation_indices.first)
                    % (animation_indices.last + 1 - animation_indices.first);
        }
    }
}
