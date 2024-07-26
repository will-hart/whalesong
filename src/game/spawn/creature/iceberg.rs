use bevoids::boids::BoidRepulsor;
use bevy::prelude::*;
use rand::Rng;

use crate::{
    game::{
        assets::{HandleMap, ImageKey},
        movement::{MoveWithVelocity, RotateToFaceMovement, WHALE_TRAVEL_SPEED},
        spawn::encounters::EncounterType,
        weather::TintWithDayNightCycle,
    },
    screen::Screen,
};

use super::Creature;

pub const SHIP_SPEED: f32 = WHALE_TRAVEL_SPEED * 0.8;

/// Denotes a ship
#[derive(Component)]
pub struct Iceberg;

pub(super) fn plugin(_app: &mut App) {
    // actually nothing :shrug:
}

/// Spawns an iceberg when `SpawnEncounter(Iceberg)` is triggered. Called by the parent creature plugin
pub(super) fn spawn_iceberg(
    commands: &mut Commands,
    win_size: Vec2,
    image_handles: &HandleMap<ImageKey>,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 9, 2, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let half_x = win_size.x / 2.;

    let mut rng = rand::thread_rng();

    commands.spawn((
        Name::new("Iceberg"),
        Creature(EncounterType::Iceberg),
        Iceberg,
        SpriteBundle {
            texture: image_handles[&ImageKey::Features].clone_weak(),
            transform: Transform::from_translation(Vec3::new(
                rng.gen_range((-half_x + 32.)..(half_x - 32.)),
                -win_size.y / 2. - 48.0,
                1.5,
            )), // move it up slightly so it obscures waves, the whale, fish, etc
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: rng.gen_range(8..14),
        },
        BoidRepulsor {
            strength: 0.6,
            range: 45.,
        },
        TintWithDayNightCycle,
        StateScoped(Screen::Playing),
        RotateToFaceMovement,
        MoveWithVelocity(Vec3::Y * 0.75 * SHIP_SPEED),
    ));
}
