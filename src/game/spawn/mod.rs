//! Handles spawning of entities. Here, we are using
//! [observers](https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.Observer.html)
//! for this, but you could also use `Events<E>` or `Commands`.

use bevy::{prelude::*, window::PrimaryWindow};
use rand::{seq::SliceRandom, Rng};
use tiny_bail::r;

use super::movement::WINDOW_DESPAWN_BUFFER;

pub mod creature;
pub mod encounters;
pub mod level;
pub mod player;

#[derive(Resource)]
pub struct WindowSize {
    size: Vec2,
}

impl WindowSize {
    pub fn set(&mut self, size: Vec2) {
        self.size = size;
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }

    pub fn half(&self) -> Vec2 {
        self.size / 2.0
    }

    pub fn clamp_to_screen_with_buffer(&self, clampee: Vec3, buffer: Val) -> Vec3 {
        let half = self.half();

        if half.is_nan() {
            warn!("Skipping clamp_to_screen as NaN screen size");
            return clampee;
        }

        let y_bound = match buffer {
            Val::Px(v) => half.y - v,
            Val::Percent(p) => (1. - p / 100.) * half.y,
            Val::Auto => half.y - 10.,
            _ => {
                warn!("Unsupported value type passed to clamp_to_screen_with_buffer, ignoring");
                half.y
            }
        };

        let x_bound = match buffer {
            Val::Px(v) => half.x - v,
            Val::Percent(p) => (1. - p / 100.) * half.x,
            Val::Auto => half.x - 10.,
            _ => {
                warn!("Unsupported value type passed to clamp_to_screen_with_buffer, ignoring");
                half.x
            }
        };

        Vec3::new(
            clampee.x.clamp(-x_bound, x_bound),
            clampee.y.clamp(-y_bound, y_bound),
            clampee.z,
        )
    }

    pub fn get_random_position(&self) -> Vec2 {
        let mut rng = rand::thread_rng();
        let half = self.half();
        Vec2::new(
            rng.gen_range(-half.x..half.x),
            rng.gen_range(-half.y..half.y),
        )
    }

    pub fn get_random_position_outside(&self) -> Vec2 {
        let mut rng = rand::thread_rng();
        let half = self.half();

        if rng.gen_bool(0.5) {
            // leave via x edge
            Vec2::new(
                *[
                    -half.x - 2. * WINDOW_DESPAWN_BUFFER,
                    half.x + 2. * WINDOW_DESPAWN_BUFFER,
                ]
                .choose(&mut rng)
                .unwrap_or(&(half.x + 2. * WINDOW_DESPAWN_BUFFER)),
                rng.gen_range(-half.y..half.y),
            )
        } else {
            // leave via y edge
            Vec2::new(
                rng.gen_range(-half.x..half.x),
                *[
                    -half.y - 2. * WINDOW_DESPAWN_BUFFER,
                    half.y + 2. * WINDOW_DESPAWN_BUFFER,
                ]
                .choose(&mut rng)
                .unwrap_or(&(half.y + 2. * WINDOW_DESPAWN_BUFFER)),
            )
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        encounters::plugin,
        level::plugin,
        player::plugin,
        creature::plugin,
    ));

    app.insert_resource(WindowSize { size: Vec2::ONE })
        .add_systems(PreUpdate, update_window_size);
}

/// Cache the size here so we have it everywhere without a song and dance
fn update_window_size(mut size: ResMut<WindowSize>, windows: Query<&Window, With<PrimaryWindow>>) {
    let w = r!(windows.get_single());
    size.set(w.size());
}
