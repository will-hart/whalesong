use bevoids::boids::{BoidCollisionGroup, BoidJitter};
use bevy::prelude::*;
use rand::{seq::SliceRandom, Rng};

use crate::{
    game::{
        animation::SpriteAnimationPlayer,
        assets::{HandleMap, ImageKey},
        spawn::encounters::EncounterType,
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

pub(super) fn plugin(_app: &mut App) {
    // nothing for now, as fish have their AI covered by Boids
}

/// Spawns a fish when `SpawnEncounter(Fish)` is triggered. Called by the parent creature plugin
pub(super) fn spawn(
    commands: &mut Commands,
    win_size: Vec2,
    image_handles: &HandleMap<ImageKey>,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 8, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let mut rng = rand::thread_rng();

    let mut player_animation = SpriteAnimationPlayer::fish();
    player_animation.set_frame(rng.gen_range(0..8));

    let (from_pos, to_pos) = get_creature_path(win_size, 64.);

    // avoid some collisions between schools, but occasionally let them interact
    let collision = [
        BoidCollisionGroup::GROUP_1,
        BoidCollisionGroup::GROUP_2,
        BoidCollisionGroup::GROUP_3,
        BoidCollisionGroup::GROUP_4,
    ]
    .choose(&mut rng)
    .unwrap_or(&BoidCollisionGroup::GROUP_18);

    let school_size = rng.gen_range(5..35);

    for fish in 0..school_size {
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
            *collision,
        ));

        if fish == 0 {
            entity_cmds.insert(LeadFish);
        }
    }
}
