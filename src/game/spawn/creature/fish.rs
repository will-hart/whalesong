use bevy::prelude::*;

use crate::{
    game::{
        animation::PlayerAnimation,
        assets::{HandleMap, ImageKey},
        movement::{MoveTowardsLocation, MoveWithWhale, RotateToFaceMovement, WHALE_TRAVEL_SPEED},
        spawn::encounters::{EncounterType, SpawnEncounter},
    },
    screen::Screen,
};

use super::{get_creature_path, Creature};

pub const FISH_SPEED: f32 = WHALE_TRAVEL_SPEED * 1.1;

/// Marker component for fish
#[derive(Component)]
pub struct Fish;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), spawn_test_fish);
}

fn spawn_test_fish(mut commands: Commands) {
    commands.trigger(SpawnEncounter {
        encounter_type: EncounterType::Fish,
    })
}

/// Spawns a fish when `SpawnEncounter(Fish)` is triggered. Called by the parent creature plugin
pub(super) fn spawn_fish(
    commands: &mut Commands,
    win_size: Vec2,
    image_handles: &HandleMap<ImageKey>,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 8, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let player_animation = PlayerAnimation::fish();

    let (from_pos, to_pos) = get_creature_path(win_size, 64.);

    commands.spawn((
        Name::new("Bird"),
        Creature(EncounterType::Bird),
        Fish,
        SpriteBundle {
            texture: image_handles[&ImageKey::Fish].clone_weak(),
            transform: Transform::from_translation(from_pos),
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: player_animation.get_atlas_index(),
        },
        player_animation,
        StateScoped(Screen::Playing),
        MoveTowardsLocation {
            speed: FISH_SPEED,
            target: to_pos,
            remove_on_arrival: true,
        },
        RotateToFaceMovement,
        MoveWithWhale,
    ));
}
