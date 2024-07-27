use bevy::prelude::*;
use rand::Rng;

use crate::{
    game::{
        animation::SpriteAnimationPlayer,
        assets::{HandleMap, ImageKey, SfxKey},
        audio::sfx::PlaySfx,
        flipper::FlipComplete,
        movement::{MoveTowardsLocation, RotateToFaceMovement, WHALE_TRAVEL_SPEED},
        spawn::{encounters::EncounterType, player::Whale, WindowSize},
        weather::TravelDistance,
    },
    screen::Screen,
};

use super::Creature;

#[derive(Resource, Default)]
pub struct BabyWhaleStatus {
    pub departure_time: f32,
    pub has_whale: bool,
}

#[derive(Component)]
pub struct BabyWhale;

const BABY_WHALE_LAG_DISTANCE: f32 = 45.;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<BabyWhaleStatus>()
        .observe(spawn_baby_on_flip);

    app.add_systems(
        FixedUpdate,
        move_baby_whale.run_if(in_state(Screen::Playing)),
    );
    app.add_systems(Update, depart_baby_whale.run_if(in_state(Screen::Playing)));
}

fn spawn_baby_on_flip(
    _trigger: Trigger<FlipComplete>,
    mut commands: Commands,
    mut baby: ResMut<BabyWhaleStatus>,
    image_handles: Res<HandleMap<ImageKey>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    whales: Query<&Transform, With<Whale>>,
) {
    // if we have a baby calculate its departure time and prevent spawning a baby on the next flip
    if baby.has_whale {
        baby.departure_time = rand::thread_rng().gen_range(35.0..55.0);
        baby.has_whale = false; // no longer have a whale
    } else {
        baby.departure_time = 0.;
        return;
    }

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 8, 8, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_animation = SpriteAnimationPlayer::baby_swimming();

    let target = whales.single().translation + Vec3::Y * BABY_WHALE_LAG_DISTANCE;

    // now spawn the baby
    let entity = commands
        .spawn((
            Name::new("Baby Whale"),
            Creature(EncounterType::BabyWhale),
            BabyWhale,
            SpriteBundle {
                texture: image_handles[&ImageKey::Creatures].clone_weak(),
                transform: Transform::from_translation(target), // spawn above existing whale
                ..Default::default()
            },
            TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: player_animation.get_atlas_index(),
            },
            player_animation,
            RotateToFaceMovement,
            MoveTowardsLocation {
                target,
                speed: WHALE_TRAVEL_SPEED,
            },
            StateScoped(Screen::Playing),
        ))
        .id();

    // play the baby noises
    commands.trigger(PlaySfx::once(SfxKey::BabyWhaleSong).with_parent(entity));
}

fn move_baby_whale(
    whales: Query<&Transform, (With<Whale>, Without<BabyWhale>)>,
    mut babies: Query<&mut MoveTowardsLocation, With<BabyWhale>>,
) {
    if let Ok(mut baby_mover) = babies.get_single_mut() {
        let whale = whales.single();
        baby_mover.target = whale.translation + Vec3::Y * BABY_WHALE_LAG_DISTANCE;
    }
}

fn depart_baby_whale(
    mut commands: Commands,
    win_size: Res<WindowSize>,
    distance: Res<TravelDistance>,
    baby_stats: Res<BabyWhaleStatus>,
    babies: Query<Entity, With<BabyWhale>>,
) {
    if let Ok(baby) = babies.get_single() {
        if baby_stats.departure_time < distance.get() {
            // get a location outside the screen
            let target = win_size.get_random_position_outside();

            // the baby can leave
            commands
                .entity(baby)
                .remove::<BabyWhale>() // its all grown up!
                .insert(MoveTowardsLocation {
                    target: target.extend(0.0),
                    speed: WHALE_TRAVEL_SPEED,
                }); // bye mum!
        }
    }
}
