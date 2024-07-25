use bevoids::boids::BoidJitter;
use bevy::prelude::*;
use rand::Rng;

use crate::{
    game::{
        animation::PlayerAnimation,
        assets::{HandleMap, ImageKey},
        spawn::encounters::{EncounterType, SpawnEncounter},
    },
    screen::Screen,
};

use super::{boid::get_default_boid, get_creature_path, Creature};

/// Marker component for fish
#[derive(Component)]
pub struct Fish;

/// The "lead fish" in the group, used for e.g. to see how far away fish are from the whale
#[derive(Component)]
pub struct LeadFish;

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

    let mut rng = rand::thread_rng();

    let mut player_animation = PlayerAnimation::fish();
    player_animation.set_frame(rng.gen_range(0..8));

    let (from_pos, to_pos) = get_creature_path(win_size, 64.);

    for fish in 0..20 {
        let mut boid = get_default_boid();
        boid.set_velocity((to_pos - from_pos).normalize() * 100.);

        let mut entity_cmds = commands.spawn((
            Name::new(format!("Fish {fish}")),
            Creature(EncounterType::Fish),
            Fish,
            SpriteBundle {
                texture: image_handles[&ImageKey::Fish].clone_weak(),
                transform: Transform::from_translation(
                    from_pos
                        + Vec3::new(rng.gen_range(-15.0..15.0), rng.gen_range(-15.0..15.0), 0.0),
                ),
                ..Default::default()
            },
            TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: player_animation.get_atlas_index(),
            },
            player_animation.clone(),
            StateScoped(Screen::Playing),
            boid,
            BoidJitter(1.3),
        ));

        if fish == 0 {
            entity_cmds.insert(LeadFish);
        }
    }
}
