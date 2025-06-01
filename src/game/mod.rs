use bevy::prelude::*;

mod animation;
pub mod level;
pub mod planet;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((level::plugin, planet::plugin, animation::plugin));
}
