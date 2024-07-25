//! Spawn the player.

use std::time::Duration;

use bevoids::boids::BoidRepulsor;
use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;

use crate::{
    game::{
        animation::{AnimationComplete, PlayerAnimation, WHALE_BREATH_FRAME_RATE},
        assets::{HandleMap, ImageKey, SfxKey},
        audio::sfx::PlaySfx,
        movement::{Movement, MovementController, WHALE_TRAVEL_SPEED},
    },
    screen::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_player);
    app.add_systems(
        FixedUpdate,
        move_in_spawning_whale.run_if(in_state(Screen::Playing)),
    );
    app.add_systems(Update, spawn_breaths.run_if(in_state(Screen::Playing)));
    app.register_type::<Whale>();
    app.init_resource::<WhaleLocation>();
}

#[derive(Event, Debug)]
pub struct SpawnPlayer;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Whale;

#[derive(Resource, Default)]
pub struct WhaleLocation {
    pub y: f32,
    pub current_rotation: f32,
    pub target_rotation: f32,
}

#[derive(Component)]
pub struct InputHelp;

#[derive(Component)]
pub struct WhaleArrivalMarker;

#[derive(Copy, Clone)]
enum BreathingPhase {
    Underwater,
    AboveWater,
}

#[derive(Component)]
struct BreathingTimer {
    timer: Timer,
    phase: BreathingPhase,
}

fn breath_bundle(
    image_handles: &HandleMap<ImageKey>,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 8, 3, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_animation = PlayerAnimation::breath();

    (
        SpriteBundle {
            texture: image_handles[&ImageKey::Creatures].clone_weak(),
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: player_animation.get_atlas_index(),
        },
        player_animation,
        StateScoped(Screen::Playing),
    )
}

fn spawn_breaths(
    mut commands: Commands,
    time: Res<Time>,
    image_handles: Res<HandleMap<ImageKey>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut breaths: Query<(Entity, &mut BreathingTimer)>,
) {
    let mut rng = rand::thread_rng();

    for (whale_entity, mut breath) in &mut breaths {
        breath.timer.tick(time.delta());

        if breath.timer.finished() {
            // what we do depends on current phase
            match &breath.phase {
                BreathingPhase::Underwater => {
                    // spawn the "breaching animation"
                    commands.entity(whale_entity).with_children(|whale| {
                        whale
                            .spawn(breath_bundle(&image_handles, &mut texture_atlas_layouts))
                            .observe(despawn_breaths);
                    });

                    breath.phase = BreathingPhase::AboveWater;
                }
                BreathingPhase::AboveWater => {
                    // play the SFX
                    commands.trigger(PlaySfx::Key(SfxKey::WhaleBreath));
                    breath.phase = BreathingPhase::Underwater;
                }
            }

            // now update the timer and restart
            let next_duration = match &breath.phase {
                BreathingPhase::Underwater => rng.gen_range(5.0..12.0),
                BreathingPhase::AboveWater => (5 * WHALE_BREATH_FRAME_RATE) as f32 / 1000.,
            };
            breath
                .timer
                .set_duration(Duration::from_secs_f32(next_duration));
            breath.timer.reset();
            breath.timer.unpause();
        }
    }
}

fn despawn_breaths(trigger: Trigger<AnimationComplete>, mut commands: Commands) {
    commands.entity(trigger.entity()).despawn();
}

fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    mut whale_pos: ResMut<WhaleLocation>,
    image_handles: Res<HandleMap<ImageKey>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 8, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_animation = PlayerAnimation::new();

    let height = match windows.get_single() {
        Ok(w) => w.height(),
        Err(_) => return,
    };
    let half_height = height / 2.0;
    whale_pos.y = half_height * 0.5;

    let start_pos = Vec3::new(0.0, half_height + 64., 0.);

    let breath_timer = BreathingTimer {
        timer: Timer::from_seconds(8.0, TimerMode::Once),
        phase: BreathingPhase::Underwater,
    };

    commands.spawn((
        Name::new("Player"),
        Whale,
        BoidRepulsor {
            strength: 10.,
            range: 80.,
        },
        SpriteBundle {
            texture: image_handles[&ImageKey::Creatures].clone_weak(),
            transform: Transform::from_translation(start_pos),
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: player_animation.get_atlas_index(),
        },
        MovementController::default(),
        Movement { speed: 420.0 },
        player_animation,
        WhaleArrivalMarker,
        StateScoped(Screen::Playing),
        breath_timer,
    ));
}

fn move_in_spawning_whale(
    mut commands: Commands,
    whale_pos: Res<WhaleLocation>,
    image_handles: Res<HandleMap<ImageKey>>,
    mut whales: Query<(Entity, &mut Transform), With<WhaleArrivalMarker>>,
) {
    for (entity, mut whale) in &mut whales {
        whale.translation.y -= WHALE_TRAVEL_SPEED;

        if whale.translation.y < whale_pos.y {
            info!("Whale has arrived, spawning helpers");
            commands
                .entity(entity)
                .remove::<WhaleArrivalMarker>()
                .with_children(|parent| {
                    parent.spawn((
                        Name::new("PlayerHelp Left"),
                        InputHelp,
                        SpriteBundle {
                            texture: image_handles[&ImageKey::Icons].clone_weak(),
                            transform: Transform::from_scale(Vec3::splat(0.5))
                                .with_translation(Vec3::new(-45., -30., 0.0)),
                            ..Default::default()
                        },
                        StateScoped(Screen::Playing),
                    ));

                    parent.spawn((
                        Name::new("PlayerHelp Right"),
                        InputHelp,
                        SpriteBundle {
                            texture: image_handles[&ImageKey::Icons].clone_weak(),
                            transform: Transform::from_scale(Vec3::splat(0.5))
                                .with_translation(Vec3::new(45., -30., 0.0))
                                .with_rotation(Quat::from_rotation_z(std::f32::consts::PI)),
                            ..Default::default()
                        },
                        StateScoped(Screen::Playing),
                    ));

                    parent.spawn((
                        Name::new("PlayerHelp Down"),
                        InputHelp,
                        SpriteBundle {
                            texture: image_handles[&ImageKey::Icons].clone_weak(),
                            transform: Transform::from_scale(Vec3::splat(0.5))
                                .with_translation(Vec3::new(0., -50., 0.0))
                                .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                            ..Default::default()
                        },
                        StateScoped(Screen::Playing),
                    ));
                });
        }
    }
}
