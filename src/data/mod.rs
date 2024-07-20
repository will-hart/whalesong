use bevy::prelude::*;
use duck::{manifest::DuckManifest, DuckBundle};
use leafwing_manifest::{
    asset_state::AssetLoadingState,
    plugin::{ManifestPlugin, RegisterManifest},
};

use crate::screen::Screen;

mod duck;

impl AssetLoadingState for Screen {
    const LOADING: Self = Self::LoadManifests;
    const PROCESSING: Self = Self::ProcessManifests;
    const READY: Self = Self::Title;
    const FAILED: Self = Self::ManifestLoadingFailed;
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(ManifestPlugin::<Screen> {
        automatically_advance_states: true,
        set_initial_state: false,
        ..default()
    })
    .register_manifest::<DuckManifest>("data/ducks.ron")
    .add_systems(OnEnter(Screen::Playing), spawn_extra_duck);
}

fn spawn_extra_duck(
    mut commands: Commands,
    manifest: Res<DuckManifest>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for duck in manifest.ducks.values() {
        commands.spawn(DuckBundle::from_manifest(duck, &mut texture_atlas_layouts));
    }
}
