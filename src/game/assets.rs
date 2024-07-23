use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
    utils::HashMap,
};
use serde::{Deserialize, Serialize};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<HandleMap<ImageKey>>();
    app.init_resource::<HandleMap<ImageKey>>();

    app.register_type::<HandleMap<SfxKey>>();
    app.init_resource::<HandleMap<SfxKey>>();

    app.register_type::<HandleMap<SoundtrackKey>>();
    app.init_resource::<HandleMap<SoundtrackKey>>();
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect, Debug, Serialize, Deserialize)]
pub enum ImageKey {
    Creatures,
    Features,
    Icons,
    Logo,
    PlayButton,
    CreditsButton,
    ExitButton,
}

impl AssetKey for ImageKey {
    type Asset = Image;
}

impl FromWorld for HandleMap<ImageKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (
                ImageKey::Icons,
                asset_server.load_with_settings(
                    "images/icons.png",
                    |settings: &mut ImageLoaderSettings| settings.sampler = ImageSampler::linear(),
                ),
            ),
            (
                ImageKey::Logo,
                asset_server
                    .load_with_settings("images/logo.png", |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::linear()
                    }),
            ),
            (
                ImageKey::PlayButton,
                asset_server.load_with_settings(
                    "images/play_button.png",
                    |settings: &mut ImageLoaderSettings| settings.sampler = ImageSampler::linear(),
                ),
            ),
            (
                ImageKey::CreditsButton,
                asset_server.load_with_settings(
                    "images/credits_button.png",
                    |settings: &mut ImageLoaderSettings| settings.sampler = ImageSampler::linear(),
                ),
            ),
            (
                ImageKey::ExitButton,
                asset_server.load_with_settings(
                    "images/exit_button.png",
                    |settings: &mut ImageLoaderSettings| settings.sampler = ImageSampler::linear(),
                ),
            ),
            (
                ImageKey::Creatures,
                asset_server.load_with_settings(
                    "images/creatures.png",
                    |settings: &mut ImageLoaderSettings| settings.sampler = ImageSampler::linear(),
                ),
            ),
            (
                ImageKey::Features,
                asset_server.load_with_settings(
                    "images/features.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::linear();
                    },
                ),
            ),
        ]
        .into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SfxKey {
    ButtonHover,
    ButtonPress,
}

impl AssetKey for SfxKey {
    type Asset = AudioSource;
}

impl FromWorld for HandleMap<SfxKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (
                SfxKey::ButtonHover,
                asset_server.load("audio/sfx/button_hover.ogg"),
            ),
            (
                SfxKey::ButtonPress,
                asset_server.load("audio/sfx/button_press.ogg"),
            ),
        ]
        .into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SoundtrackKey {
    Menu,
    Gameplay,
}

impl AssetKey for SoundtrackKey {
    type Asset = AudioSource;
}

impl FromWorld for HandleMap<SoundtrackKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (
                SoundtrackKey::Menu,
                asset_server.load("audio/soundtracks/menu_score.ogg"),
            ),
            (
                SoundtrackKey::Gameplay,
                asset_server.load("audio/soundtracks/calm_winds.ogg"),
            ),
        ]
        .into()
    }
}

pub trait AssetKey: Sized {
    type Asset: Asset;
}

#[derive(Resource, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub struct HandleMap<K: AssetKey>(HashMap<K, Handle<K::Asset>>);

impl<K: AssetKey, T> From<T> for HandleMap<K>
where
    T: Into<HashMap<K, Handle<K::Asset>>>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<K: AssetKey> HandleMap<K> {
    pub fn all_loaded(&self, asset_server: &AssetServer) -> bool {
        self.values()
            .all(|x| asset_server.is_loaded_with_dependencies(x))
    }
}
