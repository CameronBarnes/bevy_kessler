use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use crate::asset_tracking::LoadResource;

use super::animation::PlanetAnimation;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<PlanetAssets>();
    app.load_resource::<PlanetAssets>();
}

pub fn planet(
    planet_assets: &PlanetAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(150), 40, 30, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let planet_animation = PlanetAnimation::new();

    (
        Name::new("Planet"),
        Planet,
        Sprite {
            image: planet_assets.spritesheet.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: planet_animation.get_atlas_index(),
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(2.)),
        planet_animation,
    )
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
struct Planet;

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlanetAssets {
    #[dependency]
    spritesheet: Handle<Image>,
}

impl FromWorld for PlanetAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            spritesheet: assets.load_with_settings(
                "images/planet_spritesheet.png",
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        }
    }
}
