use std::time::Duration;

use bevy::prelude::*;

use crate::{AppSystems, PausableSystems};

use super::planet::PlanetAssets;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<PlanetAnimation>();
    app.add_systems(
        Update,
        (
            update_animation_timer.in_set(AppSystems::TickTimers),
            update_animation_atlas
                .run_if(resource_exists::<PlanetAssets>)
                .in_set(AppSystems::Update),
        )
            .in_set(PausableSystems),
    );
}

fn update_animation_timer(time: Res<Time>, mut animation_query: Query<&mut PlanetAnimation>) {
    for mut animation in &mut animation_query {
        animation.update_timer(time.delta());
    }
}

/// Update the texture atlas to reflect changes in the animation.
fn update_animation_atlas(mut query: Query<(&PlanetAnimation, &mut Sprite)>) {
    for (animation, mut sprite) in &mut query {
        let Some(atlas) = sprite.texture_atlas.as_mut() else {
            continue;
        };
        if animation.changed() {
            atlas.index = animation.get_atlas_index();
        }
    }
}

/// Component that tracks Planet animation state.
/// It is tightly bound to the texture atlas we use.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlanetAnimation {
    timer: Timer,
    frame: usize,
}

impl PlanetAnimation {
    const FRAMES: usize = 40 * 30;
    const DURATION: Duration = Duration::from_millis(40);

    pub fn new() -> Self {
        Self {
            timer: Timer::new(Self::DURATION, TimerMode::Repeating),
            frame: 0,
        }
    }

    /// Update animation timers.
    pub fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if self.timer.finished() {
            self.frame = (self.frame + 1) % Self::FRAMES;
        }
    }

    /// Whether animation changed this tick.
    pub fn changed(&self) -> bool {
        self.timer.finished()
    }

    /// Return sprite index in the atlas.
    pub fn get_atlas_index(&self) -> usize {
        self.frame
    }
}
