//! Handles spawning of entities. Here, we are using
//! [observers](https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.Observer.html)
//! for this, but you could also use `Events<E>` or `Commands`.

use bevy::{prelude::*, window::PrimaryWindow};

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
    match windows.get_single() {
        Ok(w) => size.set(w.size()),
        Err(_) => {
            warn!("no primary window found, skipping size update");
        }
    }
}
