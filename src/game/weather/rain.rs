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

impl Raininess {
    /// updates the rain status and returns true if the status changed
    pub fn update(&mut self, distance: &TravelDistance, delta: Range<f32>) -> bool {
        let mut rng = rand::thread_rng();
        self.factor = (self.factor + rng.gen_range(delta)).clamp(0.0, 1.0);
        let was_raining = self.is_raining();

        if self.factor > RAIN_THRESHOLD && self.time_rain_ends < 0.01 {
            // start raining
            self.time_rain_ends = distance.future_range(16.0..25.0); // rain for this many seconds
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

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Raininess {
        factor: 0.,
        time_rain_ends: 0.,
    });
    app.add_systems(OnEnter(Screen::Playing), reset_raininess);
    app.add_systems(
        Update,
        (update_raininess, spawn_rain_drops).run_if(in_state(Screen::Playing)),
    );
    app.observe(handle_rain_changed);
}

pub fn reset_raininess(mut raininess: ResMut<Raininess>) {
    raininess.reset();
}

pub fn update_raininess(
    mut commands: Commands,
    distance: Res<TravelDistance>,
    mut raininess: ResMut<Raininess>,
) {
    if raininess.update(&distance, -0.001..0.0015) {
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
    rain_spawn_period: f32,
    next_spawn: f32,
}

fn handle_rain_changed(
    trigger: Trigger<RainChanged>,
    mut commands: Commands,
    rains: Query<(Entity, &Children), With<Rain>>,
    audio_children: Query<Entity, With<Handle<AudioSource>>>,
) {
    let evt = trigger.event();

    if evt.is_raining {
        info!("Spawning rain");
        let entity = commands
            .spawn(Rain {
                rain_spawn_period: 0.1,
                next_spawn: 0.,
            })
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
        info!("Despawning rain");
        for (rain_entity, children) in &rains {
            for child in children {
                if let Ok(audio) = audio_children.get(*child) {
                    info!("--> adding FadeOut component");
                    commands.entity(rain_entity).clear_children().despawn();
                    commands.entity(audio).insert(FadeOut {
                        rate_per_second: 1.,
                    });
                }
            }
        }
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

        rain.next_spawn = rain.next_spawn + rain.rain_spawn_period;
    }
}
