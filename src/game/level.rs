//! Spawn the main level.

use bevy::prelude::*;

use crate::{
    asset_tracking::LoadResource,
    audio::{Playlist, StartMusic},
    screens::Screen,
};

use super::planet::{planet, PlanetAssets};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<LevelAssets>();
    app.load_resource::<LevelAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    music: Vec<Handle<AudioSource>>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: vec![
                assets.load("audio/music/01 Kevin MacLeod - Impact Prelude.mp3"),
                assets.load("audio/music/02 Kevin MacLeod - Impact Andante.mp3"),
                assets.load("audio/music/03 Kevin MacLeod - Impact Moderato.mp3"),
                assets.load("audio/music/06 Kevin MacLeod - Impact Lento.mp3"),
            ],
        }
    }
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    planet_assets: Res<PlanetAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
        children![
            planet(&planet_assets, &mut texture_atlas_layouts),
            (
                Name::new("Gameplay Music Playlist"),
                Playlist::new(level_assets.music.clone())
            )
        ],
    ));

    commands.trigger(StartMusic);
}
