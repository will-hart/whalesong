//! Game mechanics and content.

use bevy::prelude::*;
use bevy_tween::DefaultTweenPlugins;

mod animation;
pub mod assets;
pub mod audio;
mod movement;
pub mod spawn;
mod weather;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(DefaultTweenPlugins);

    app.add_plugins((
        animation::plugin,
        audio::plugin,
        assets::plugin,
        movement::plugin,
        spawn::plugin,
        weather::plugin,
    ));
}
