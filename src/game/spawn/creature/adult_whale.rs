use bevy::prelude::*;
use rand::Rng;

use crate::{
    game::{
        animation::AnimationPlayer,
        assets::{HandleMap, ImageKey, SfxKey},
        audio::sfx::PlaySfx,
        flipper::FlipComplete,
        movement::{MoveWithVelocity, WHALE_TRAVEL_SPEED},
        spawn::{
            encounters::{EncounterTimers, EncounterType},
            player::Whale,
        },
        weather::{TravelDirection, TravelDistance},
    },
    screen::Screen,
};

use super::{baby_whale::BabyWhaleStatus, bird::Curious, get_creature_path, Creature};

#[derive(Component)]
pub struct AdultWhale;

pub(super) fn plugin(app: &mut App) {
    app.observe(set_adult_spawn_time);
    app.add_systems(
        Update,
        adult_whale_gain_curiosity.run_if(in_state(Screen::Playing)),
    );
    app.add_systems(
        FixedUpdate,
        adult_whale_follows_player_whale.run_if(in_state(Screen::Playing)),
    );
}

fn set_adult_spawn_time(
    _trigger: Trigger<FlipComplete>,
    distance: Res<TravelDistance>,
    mut encounter_timers: ResMut<EncounterTimers>,
) {
    if !matches!(distance.travel_direction(), TravelDirection::North) {
        // only spawn adults on the northward journey
        return;
    }

    info!("Considering whale spawn");
    let mut rng = rand::thread_rng();

    if distance.get_flip_number() == 1 || rng.gen_bool(0.5) {
        let time = rng.gen_range(3.0..4.0);
        info!("Planning an adult whale arrival at {time}");
        encounter_timers.set_adult_spawn(time);
    }
}

pub const WHALE_CURIOSITY_DISTANCE: f32 = 80.;

/// Looks at adult whales and works out if they're close enough to a whale to get curious about it
fn adult_whale_gain_curiosity(
    mut commands: Commands,
    mut baby_status: ResMut<BabyWhaleStatus>,
    whales: Query<&Transform, With<Whale>>,
    adults: Query<(Entity, &Transform), (With<AdultWhale>, Without<Curious>)>,
) {
    if adults.is_empty() || whales.is_empty() {
        return;
    }

    let whale = whales.single();
    let mut rng = rand::thread_rng();
    let target = Vec3::new(
        whale.translation.x + rng.gen_range(-20.0..20.0),
        whale.translation.y + rng.gen_range(-20.0..20.0),
        0.,
    );

    for (whale, tx) in &adults {
        let delta_pos = target - tx.translation;
        if delta_pos.length_squared() < WHALE_CURIOSITY_DISTANCE * WHALE_CURIOSITY_DISTANCE {
            info!("whale {whale:?} is curious");

            commands.trigger(PlaySfx::once(SfxKey::AdultWhaleSong));

            baby_status.has_whale = true;

            commands.entity(whale).insert((
                Curious {
                    // marks them for the curiosity AI system
                    until: f32::MAX,
                },
                MoveWithVelocity(delta_pos.normalize_or_zero() * WHALE_TRAVEL_SPEED),
            ));
        }
    }
}

fn adult_whale_follows_player_whale(
    whales: Query<&Transform, With<Whale>>,
    mut adults: Query<(&Transform, &mut MoveWithVelocity), (With<AdultWhale>, With<Curious>)>,
) {
    if adults.is_empty() || whales.is_empty() {
        return;
    }

    let whale = whales.single();
    let mut rng = rand::thread_rng();

    for (whale_tx, mut movement) in &mut adults {
        movement.0 = (Vec3::new(
            whale.translation.x + rng.gen_range(-20.0..20.0),
            whale.translation.y + rng.gen_range(-20.0..20.0),
            0.,
        ) - whale_tx.translation)
            .normalize_or_zero()
            * WHALE_TRAVEL_SPEED;
    }
}

/// Spawns an adult whale when `SpawnEncounter(Fish)` is triggered. Called by the parent creature plugin.
pub(super) fn spawn(
    commands: &mut Commands,
    win_size: Vec2,
    image_handles: &HandleMap<ImageKey>,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 8, 8, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let mut player_animation = AnimationPlayer::new();
    player_animation.set_frame_interval(190.);

    let (from_pos, to_pos) = get_creature_path(win_size, 64.);

    // now spawn the baby
    commands.spawn((
        Name::new("Adult Whale"),
        Creature(EncounterType::AdultWhale),
        AdultWhale,
        SpriteBundle {
            texture: image_handles[&ImageKey::Creatures].clone_weak(),
            transform: Transform::from_translation(from_pos),
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: player_animation.get_atlas_index(),
        },
        player_animation,
        StateScoped(Screen::Playing),
        MoveWithVelocity((to_pos - from_pos).normalize() * WHALE_TRAVEL_SPEED),
    ));
}