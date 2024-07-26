use bevy::prelude::*;

use crate::{
    game::{
        animation::PlayerAnimation,
        assets::{HandleMap, ImageKey, SfxKey},
        audio::sfx::PlaySfx,
        movement::{MoveTowardsLocation, RotateToFaceMovement, WHALE_TRAVEL_SPEED},
        spawn::encounters::{EncounterType, SpawnEncounter},
        weather::TintWithDayNightCycle,
    },
    screen::Screen,
};

use super::{get_creature_path, Creature};

pub const SHIP_SPEED: f32 = WHALE_TRAVEL_SPEED * 0.8;

/// Denotes a ship
#[derive(Component)]
pub struct Ship;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), spawn_test_ship);
}

fn spawn_test_ship(mut commands: Commands) {
    commands.trigger(SpawnEncounter {
        encounter_type: EncounterType::Ship,
    });
}

/// Spawns a ship when `SpawnEncounter(Ship)` is triggered. Called by the parent creature plugin
pub(super) fn spawn_ship(
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
            StateScoped(Screen::Playing),
            RotateToFaceMovement,
            MoveTowardsLocation {
                speed: SHIP_SPEED,
                target: to_pos,
                remove_on_arrival: true,
            },
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