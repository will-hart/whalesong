use bevy::prelude::*;
use manifest::DuckData;

use crate::{game::PlayerAnimation, screen::Screen};

pub(crate) mod manifest;

#[derive(Component)]
pub struct Duck;

#[derive(Bundle)]
pub struct DuckBundle {
    pub duck: Duck,
    pub name: Name,
    pub sprite: SpriteBundle,
    pub atlas: TextureAtlas,
    pub animation: PlayerAnimation,
    pub scope: StateScoped<Screen>,
}

impl DuckBundle {
    pub fn from_manifest(
        data: &DuckData,
        texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    ) -> Self {
        let layout =
            TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let player_animation = PlayerAnimation::new();

        Self {
            duck: Duck,
            name: Name::new(data.name.clone()),
            sprite: SpriteBundle {
                texture: data.sprite.clone(),
                transform: Transform {
                    translation: data.position.clone().extend(0.0),
                    scale: data.scale.clone(),
                    ..default()
                },
                ..default()
            },
            atlas: TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: player_animation.get_atlas_index(),
            },
            animation: player_animation,
            scope: StateScoped(Screen::Playing),
        }
    }
}
