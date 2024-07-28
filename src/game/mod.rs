//! Game mechanics and content.

use bevy::prelude::*;

mod animation;
pub mod assets;
pub mod audio;
mod flipper;
mod movement;
pub mod spawn;
mod weather;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        audio::plugin,
        assets::plugin,
        flipper::plugin,
        movement::plugin,
        spawn::plugin,
        weather::plugin,
    ));
}
