use {bevy::prelude::*, std::time::Duration};

#[derive(Component, PartialEq, Default, Clone)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Clone, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub trait AnimationState {
    fn indices(self) -> AnimationIndices;
    fn timer(self) -> AnimationTimer;
}

impl AnimationTimer {
    pub fn new(duration: Duration) -> Self {
        Self(Timer::new(duration, TimerMode::Repeating))
    }

    pub fn zero() -> Self {
        Self::default()
    }
}

impl Default for AnimationTimer {
    fn default() -> Self {
        Self(Timer::new(
            Duration::from_secs_f32(0.),
            TimerMode::Repeating,
        ))
    }
}

pub fn adjust_sprite_indices(
    mut animation_qry: Query<(&mut TextureAtlas, &AnimationIndices), Changed<AnimationIndices>>,
) {
    for (mut tex_atlas, animation_indices) in &mut animation_qry {
        tex_atlas.index = animation_indices.first;
    }
}

fn animate_sprites(
    time: Res<Time>,
    mut animation_qry: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (animation_indices, mut animation_timer, mut tex_atlas) in &mut animation_qry {
        animation_timer.tick(time.delta());
        if animation_timer.just_finished() {
            tex_atlas.index = animation_indices.first
                + (tex_atlas.index + 1 - animation_indices.first)
                    % (animation_indices.last + 1 - animation_indices.first);
        }
    }
}

pub fn animation_plugin(app: &mut App) {
    app.add_systems(Update, (adjust_sprite_indices, animate_sprites).chain());
}
