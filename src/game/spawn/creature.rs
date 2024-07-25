//! Spawn the player.

use bevy::prelude::*;
use rand::Rng;

use crate::game::assets::{HandleMap, ImageKey};

use super::{
    encounters::{EncounterType, SpawnEncounter},
    WindowSize,
};

mod bird;
mod fish;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((bird::plugin, fish::plugin));
    app.observe(spawn_creature);
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Creature(pub EncounterType);

fn spawn_creature(
    trigger: Trigger<SpawnEncounter>,
    mut commands: Commands,
    win_size: Res<WindowSize>,
    image_handles: Res<HandleMap<ImageKey>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let size = win_size.size();

    match trigger.event().encounter_type {
        EncounterType::Bird => {
            bird::spawn_bird(
                &mut commands,
                size,
                &image_handles,
                &mut texture_atlas_layouts,
            );
        }
        EncounterType::Fish => {
            fish::spawn_fish(
                &mut commands,
                size,
                &image_handles,
                &mut texture_atlas_layouts,
            );
        }
    }
}

/// Returns the ends of a path for spawning a creature
fn get_creature_path(window_size: Vec2, sprite_size: f32) -> (Vec3, Vec3) {
    let mut rng = rand::thread_rng();
    let half_size = window_size / 2.0;

    // check if we're going from L-R or U-D
    let (a, b) = if rng.gen_bool(0.5) {
        // L-R
        (
            Vec2::new(
                -half_size.x - sprite_size,
                rng.gen_range(-half_size.y + sprite_size..half_size.y - sprite_size),
            ),
            Vec2::new(
                half_size.x + sprite_size,
                rng.gen_range(-half_size.y + sprite_size..half_size.y - sprite_size),
            ),
        )
    } else {
        // U-D
        (
            Vec2::new(
                rng.gen_range(-half_size.x + sprite_size..half_size.x - sprite_size),
                -half_size.y - sprite_size,
            ),
            Vec2::new(
                rng.gen_range(-half_size.x + sprite_size..half_size.x - sprite_size),
                half_size.y + sprite_size,
            ),
        )
    };

    // check if we should switch them
    if rng.gen_bool(0.5) {
        (a.extend(0.), b.extend(0.))
    } else {
        (b.extend(0.), a.extend(0.))
    }
}
