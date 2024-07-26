use std::ops::Range;

use bevy::prelude::*;
use rand::Rng;

use crate::{
    game::{
        animation::{despawn_when_animation_complete, PlayerAnimation},
        assets::{HandleMap, ImageKey, SfxKey},
        audio::sfx::{FadeIn, FadeOut, PlaySfx},
        spawn::WindowSize,
    },
    screen::Screen,
};

use super::TravelDistance;

#[derive(Resource, Debug)]
pub struct Raininess {
    factor: f32,
    time_rain_ends: f32,
}

#[derive(Event)]
pub struct RainChanged {
    pub is_raining: bool,
}

const RAIN_THRESHOLD: f32 = 0.5;

const RAIN_TO_SNOW_DISTANCE: f32 = 120.;

const RAIN_MIN_DURATION: f32 = 16.0;
const RAIN_MAX_DURATION: f32 = 25.0;
const RAIN_MIN_GROWTH: f32 = 0.0;
const RAIN_MAX_GROWTH: f32 = 0.0015;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Raininess {
        factor: 0.,
        time_rain_ends: 0.,
    });
    app.add_systems(OnEnter(Screen::Playing), reset_raininess);
    app.add_systems(
        Update,
        (
            update_raininess,
            spawn_rain_drops,
            spawn_snow_flakes,
            animate_snow_flakes,
        )
            .run_if(in_state(Screen::Playing)),
    );
    app.observe(handle_rain_changed);
}

impl Raininess {
    /// updates the rain status and returns true if the status changed
    pub fn update(&mut self, distance: &TravelDistance, delta: Range<f32>) -> bool {
        let mut rng = rand::thread_rng();
        self.factor = (self.factor + rng.gen_range(delta)).clamp(0.0, 1.0);
        let was_raining = self.is_raining();

        if self.factor > RAIN_THRESHOLD && self.time_rain_ends < 0.01 {
            // start raining
            self.time_rain_ends = distance.future_range(RAIN_MIN_DURATION..RAIN_MAX_DURATION);
        // rain for this many seconds
        } else if self.time_rain_ends > 0. && self.time_rain_ends < distance.get() {
            // stop raining
            self.reset();
        }

        self.is_raining() != was_raining
    }

    pub fn is_raining(&self) -> bool {
        self.time_rain_ends > 0.
    }

    pub fn factor(&self) -> f32 {
        self.factor
    }

    pub fn reset(&mut self) {
        self.factor = 0.;
        self.time_rain_ends = 0.;
    }
}

pub fn reset_raininess(mut raininess: ResMut<Raininess>) {
    raininess.reset();
}

pub fn update_raininess(
    mut commands: Commands,
    distance: Res<TravelDistance>,
    mut raininess: ResMut<Raininess>,
) {
    if raininess.update(&distance, RAIN_MIN_GROWTH..RAIN_MAX_GROWTH) {
        commands.trigger(RainChanged {
            is_raining: raininess.is_raining(),
        });
        warn!(
            "Raininess: {}, {} raining",
            raininess.factor(),
            if raininess.is_raining() { "is" } else { "not" }
        );
    }
}

#[derive(Component)]
pub struct Rain {
    spawn_period: f32,
    next_spawn: f32,
}

#[derive(Component)]
pub struct Snow {
    spawn_period: f32,
    next_spawn: f32,
}

#[derive(Component)]
pub struct Precipitation;

fn handle_rain_changed(
    trigger: Trigger<RainChanged>,
    distance: Res<TravelDistance>,
    mut commands: Commands,
    precips: Query<(Entity, &Children, Option<&Rain>), With<Rain>>,
    audio_children: Query<Entity, With<Handle<AudioSource>>>,
) {
    let evt = trigger.event();

    if evt.is_raining {
        if distance.get() < RAIN_TO_SNOW_DISTANCE {
            info!("Spawning rain");
            let entity = commands
                .spawn((
                    Rain {
                        spawn_period: 0.1,
                        next_spawn: 0.,
                    },
                    Precipitation,
                ))
                .id();

            commands.trigger(
                PlaySfx::once(SfxKey::RainAmbient)
                    .with_parent(entity)
                    .with_volume(0.1)
                    .with_fade_in(FadeIn {
                        final_volume: 2.5,
                        rate_per_second: 1.0,
                    }),
            );
        } else {
            info!("Spawning snow");
            commands.spawn((
                SpatialBundle::default(),
                Snow {
                    spawn_period: 0.15,
                    next_spawn: 0.,
                },
                Precipitation,
            ));
        };
    } else {
        for (entity, children, is_rain) in &precips {
            for child in children {
                if is_rain.is_some() {
                    info!("Despawning rain");
                    if let Ok(audio) = audio_children.get(*child) {
                        info!("--> adding FadeOut component");
                        commands.entity(entity).clear_children();
                        commands.entity(audio).insert(FadeOut {
                            rate_per_second: 1.,
                        });
                    }
                } else {
                    info!("Despawning snow");
                }

                commands.entity(entity).despawn();
            }
        }

        info!("Despawning snow");
    }
}

fn spawn_rain_drops(
    mut commands: Commands,
    distance: Res<TravelDistance>,
    image_handles: Res<HandleMap<ImageKey>>,
    win_size: Res<WindowSize>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut rains: Query<&mut Rain>,
) {
    for mut rain in &mut rains {
        if rain.next_spawn > distance.get() {
            continue;
        }
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 8, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let player_animation = PlayerAnimation::raindrop();

        let pos = win_size.get_random_position();

        commands
            .spawn((
                SpriteBundle {
                    texture: image_handles[&ImageKey::RainDrop].clone_weak(),
                    transform: Transform::from_translation(pos.extend(0.0)),
                    ..Default::default()
                },
                TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: player_animation.get_atlas_index(),
                },
                player_animation,
                StateScoped(Screen::Playing),
            ))
            .observe(despawn_when_animation_complete);

        rain.next_spawn = rain.next_spawn + rain.spawn_period;
    }
}

#[derive(Component)]
pub struct Snowflake {
    shrink_per_frame: f32,
    velocity: Vec3,
}

fn spawn_snow_flakes(
    mut commands: Commands,
    distance: Res<TravelDistance>,
    image_handles: Res<HandleMap<ImageKey>>,
    win_size: Res<WindowSize>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut snows: Query<(Entity, &mut Snow)>,
) {
    let mut rng = rand::thread_rng();

    for (entity, mut snow) in &mut snows {
        if snow.next_spawn > distance.get() {
            continue;
        }
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 8, 2, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        let pos = win_size.get_random_position();

        commands
            .spawn((
                SpriteBundle {
                    texture: image_handles[&ImageKey::RainDrop].clone_weak(),
                    transform: Transform::from_translation(pos.extend(0.0))
                        .with_scale(Vec3::splat(rng.gen_range(0.5..0.75))),
                    ..Default::default()
                },
                TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: rng.gen_range(8..15),
                },
                StateScoped(Screen::Playing),
                Snowflake {
                    shrink_per_frame: 0.001,
                    velocity: Vec3::new(
                        rng.gen_range(-10.0..10.0),
                        rng.gen_range(-10.0..10.0),
                        0.0,
                    )
                    .normalize_or_zero(),
                },
            ))
            .set_parent(entity);

        snow.next_spawn = snow.next_spawn + snow.spawn_period;
    }
}

fn animate_snow_flakes(
    mut commands: Commands,
    time: Res<Time>,
    mut snowflakes: Query<(Entity, &mut Transform, &mut Snowflake)>,
) {
    if snowflakes.is_empty() {
        return;
    }

    let mut rng = rand::thread_rng();

    for (entity, mut tx, mut snowflake) in &mut snowflakes {
        tx.translation += 30.0 * snowflake.velocity * time.delta_seconds();
        snowflake.velocity = (snowflake.velocity
            + Vec3::new(rng.gen_range(-0.1..0.1), rng.gen_range(-0.1..0.1), 0.0))
        .normalize_or_zero();

        tx.rotate(Quat::from_axis_angle(Vec3::Z, 0.3 * time.delta_seconds()));
        tx.scale = Vec3::splat(tx.scale.x - snowflake.shrink_per_frame);

        if tx.scale.x < 0.01 {
            commands.entity(entity).despawn();
        }
    }
}
