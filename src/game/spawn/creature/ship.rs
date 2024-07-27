use bevoids::boids::BoidRepulsor;
use bevy::prelude::*;

use crate::{
    game::{
        animation::PlayerAnimation,
        assets::{HandleMap, ImageKey, SfxKey},
        audio::sfx::PlaySfx,
        movement::{MoveWithVelocity, RotateToFaceMovement, WHALE_TRAVEL_SPEED},
        spawn::encounters::EncounterType,
        weather::TintWithDayNightCycle,
    },
    screen::Screen,
};

use super::{get_creature_path, Creature};

pub const SHIP_SPEED: f32 = WHALE_TRAVEL_SPEED * 0.8;

/// Denotes a ship
#[derive(Component)]
pub struct Ship;

pub(super) fn plugin(_app: &mut App) {
    // actually nothing :shrug:
}

/// Spawns a ship when `SpawnEncounter(Ship)` is triggered. Called by the parent creature plugin
pub(super) fn spawn(
    commands: &mut Commands,
    win_size: Vec2,
    image_handles: &HandleMap<ImageKey>,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(64, 128), 5, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let player_animation = PlayerAnimation::ship();

    let (from_pos, to_pos) = get_creature_path(win_size, 64.);

    let entity = commands
        .spawn((
            Name::new("Ship"),
            Creature(EncounterType::Ship),
            Ship,
            SpriteBundle {
                texture: image_handles[&ImageKey::Ships].clone_weak(),
                transform: Transform::from_translation(from_pos + Vec3::Z), // move it up slightly so it obscures waves, the whale, fish, etc

                ..Default::default()
            },
            TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: player_animation.get_atlas_index(),
            },
            player_animation,
            BoidRepulsor {
                strength: 0.5,
                range: 45.,
            },
            StateScoped(Screen::Playing),
            RotateToFaceMovement,
            MoveWithVelocity((to_pos - from_pos).normalize_or_zero() * SHIP_SPEED),
        ))
        .with_children(|parent_ship| {
            // spawn the ship outline underneath, marking it for tinting with day-night cycle
            parent_ship.spawn((
                SpriteBundle {
                    texture: image_handles[&ImageKey::Ships].clone_weak(),
                    transform: Transform::from_translation(-0.5 * Vec3::Z), // move it down slightly so it obscures waves but is under the ship
                    ..Default::default()
                },
                TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: 4, // I happen to know this is the outline index :shrug:
                },
                TintWithDayNightCycle,
            ));
        })
        .id();

    commands.trigger(PlaySfx::looped(SfxKey::ShipAmbient).with_parent(entity));
}
