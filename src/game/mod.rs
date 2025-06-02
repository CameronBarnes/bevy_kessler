use bevy::prelude::*;

mod animation;
mod asteroids;
pub mod level;
mod orbit;
pub mod planet;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        level::plugin,
        planet::plugin,
        animation::plugin,
        asteroids::plugin,
        orbit::plugin,
    ));
}
