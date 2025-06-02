use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};
use rand::Rng;

use crate::asset_tracking::LoadResource;

use super::orbit::Orbit;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<AsteroidAssets>();
    app.load_resource::<AsteroidAssets>();
}

#[derive(Component, Reflect, Debug, Clone, Copy)]
#[reflect(Component)]
pub enum AsteroidSize {
    Dust,
    Small,
    Large,
}

pub fn asteroid(size: AsteroidSize, angle: f32, distance: f32, speed: f32, sprite: Sprite) -> impl Bundle {
    let orbit = Orbit::new(distance, angle, speed);
    let scale = match size {
        AsteroidSize::Dust => rand::thread_rng().gen_range(0.01..=0.1),
        AsteroidSize::Small => rand::thread_rng().gen_range(0.1..=0.4),
        AsteroidSize::Large => rand::thread_rng().gen_range(0.4..1.),
    };

    (
        Name::new("Asteroid"),
        Transform::from_scale(Vec3::splat(scale)).with_translation(orbit.to_xy().extend(1.)),
        orbit,
        sprite,
    )
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct AsteroidAssets {
    #[dependency]
    spritesheet: Handle<Image>,
    #[dependency]
    atlas_layout: Handle<TextureAtlasLayout>,
}

impl AsteroidAssets {
    const NUM_SMALL_TEXTURES: usize = 13;
    const NUM_LARGE_TEXTURES: usize = 11;
    pub fn get_sprite(&self, size: AsteroidSize) -> Sprite {
        Sprite {
            image: self.spritesheet.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: self.atlas_layout.clone(),
                index: match size {
                    AsteroidSize::Dust | AsteroidSize::Small => {
                        rand::thread_rng().gen_range(0..Self::NUM_SMALL_TEXTURES)
                    }
                    AsteroidSize::Large => {
                        rand::thread_rng().gen_range(0..Self::NUM_LARGE_TEXTURES)
                    }
                },
            }),
            ..default()
        }
    }
}

impl FromWorld for AsteroidAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        let spritesheet = assets.load_with_settings(
            "images/Asteroid_spritesheet.png",
            |settings: &mut ImageLoaderSettings| {
                // Use `nearest` image sampling to preserve pixel art style.
                settings.sampler = ImageSampler::nearest();
            },
        );
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(100), 13, 2, None, None);
        let atlas_layout = assets.add(layout);
        Self {
            spritesheet,
            atlas_layout,
        }
    }
}
