//! A splash screen that plays briefly at startup.

use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
};

use super::Screen;
use crate::{ui::prelude::*, AppSet};

#[derive(Component)]
struct SplashContainer;

#[derive(Component)]
struct SplashScreenImage;

pub(super) fn plugin(app: &mut App) {
    // Spawn splash screen.
    app.insert_resource(ClearColor(SPLASH_BACKGROUND_COLOR));
    app.add_systems(OnEnter(Screen::Splash), spawn_splash);

    // Animate splash screen.
    app.add_systems(
        Update,
        (
            tick_fade_in_out.in_set(AppSet::TickTimers),
            (
                apply_fade_in_out_to_backgrounds,
                apply_fade_in_out_to_ui_images,
            )
                .in_set(AppSet::Update),
        ),
    );

    // Add splash timer.
    app.register_type::<SplashTimer>();
    app.add_systems(OnEnter(Screen::Splash), insert_splash_timer);
    app.add_systems(OnExit(Screen::Splash), remove_splash_timer);
    app.add_systems(
        Update,
        (
            tick_splash_timer.in_set(AppSet::TickTimers),
            check_splash_timer.in_set(AppSet::Update),
        )
            .run_if(in_state(Screen::Splash)),
    );
}

const SPLASH_BACKGROUND_COLOR: Color = Color::srgb(0.08, 0.08, 0.08);
const SPLASH_DURATION_SECS: f32 = 1.8;
const SPLASH_FADE_DURATION_SECS: f32 = 0.6;

fn splash_image_bundle(asset_server: &AssetServer, path: &'static str) -> impl Bundle {
    (
        Name::new("Splash image"),
        ImageBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                width: Val::Percent(70.0),
                ..default()
            },
            image: UiImage::new(asset_server.load_with_settings(
                // This should be an embedded asset for instant loading, but that is
                // currently [broken on Windows Wasm builds](https://github.com/bevyengine/bevy/issues/14246).
                path,
                |settings: &mut ImageLoaderSettings| {
                    // Make an exception for the splash image in case
                    // `ImagePlugin::default_nearest()` is used for pixel art.
                    settings.sampler = ImageSampler::linear();
                },
            )),
            ..default()
        },
        UiImageFadeInOut {
            total_duration: SPLASH_DURATION_SECS,
            fade_duration: SPLASH_FADE_DURATION_SECS,
            t: 0.0,
        },
        SplashScreenImage,
    )
}

fn spawn_splash(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .ui_root()
        .insert((
            Name::new("Splash screen"),
            BackgroundColor(SPLASH_BACKGROUND_COLOR),
            StateScoped(Screen::Splash),
            SplashContainer,
        ))
        .with_children(|children| {
            children.spawn(splash_image_bundle(&asset_server, "images/splash.png"));
        });
}

#[derive(Event)]
pub struct UiFadeComplete;

#[derive(Component, Reflect, Clone, Copy)]
#[reflect(Component)]
pub struct UiImageFadeInOut {
    /// Total duration in seconds.
    total_duration: f32,
    /// Fade duration in seconds.
    fade_duration: f32,
    /// Current progress in seconds, between 0 and [`Self::total_duration`].
    t: f32,
}

impl UiImageFadeInOut {
    pub fn new(fade_duration: f32, total_duration: f32) -> Self {
        Self {
            total_duration,
            fade_duration,
            t: 0.,
        }
    }

    fn alpha(&self) -> f32 {
        // Normalize by duration.
        let t = (self.t / self.total_duration).clamp(0.0, 1.0);
        let fade = self.fade_duration / self.total_duration;

        // Regular trapezoid-shaped graph, flat at the top with alpha = 1.0.
        ((1.0 - (2.0 * t - 1.0).abs()) / fade).min(1.0)
    }
}

fn tick_fade_in_out(
    mut commands: Commands,
    time: Res<Time>,
    mut animation_query: Query<(Entity, &mut UiImageFadeInOut)>,
) {
    for (anim_entity, mut anim) in &mut animation_query {
        let prev = anim.t;

        anim.t += time.delta_seconds();

        if prev <= anim.total_duration && anim.t >= anim.total_duration {
            commands.trigger_targets(UiFadeComplete, anim_entity);
        }
    }
}

fn apply_fade_in_out_to_ui_images(mut animation_query: Query<(&UiImageFadeInOut, &mut UiImage)>) {
    for (anim, mut image) in &mut animation_query {
        image.color.set_alpha(anim.alpha())
    }
}

fn apply_fade_in_out_to_backgrounds(
    mut animation_query: Query<(&UiImageFadeInOut, &mut BackgroundColor)>,
) {
    for (anim, mut background) in &mut animation_query {
        background.0.set_alpha(anim.alpha())
    }
}

#[derive(Resource, Debug, Clone, PartialEq, Reflect)]
#[reflect(Resource)]
struct SplashTimer(Timer);

impl Default for SplashTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(SPLASH_DURATION_SECS, TimerMode::Once))
    }
}

fn insert_splash_timer(mut commands: Commands) {
    commands.init_resource::<SplashTimer>();
}

fn remove_splash_timer(mut commands: Commands) {
    commands.remove_resource::<SplashTimer>();
}

fn tick_splash_timer(time: Res<Time>, mut timer: ResMut<SplashTimer>) {
    timer.0.tick(time.delta());
}

fn check_splash_timer(
    mut commands: Commands,
    mut timer: ResMut<SplashTimer>,
    mut next_screen: ResMut<NextState<Screen>>,
    asset_server: Res<AssetServer>,
    mut splash_screen: Local<u8>,
    containers: Query<Entity, With<SplashContainer>>,
    screens: Query<Entity, With<SplashScreenImage>>,
) {
    if !timer.0.just_finished() {
        return;
    }

    // if we've shown both splash screens, skip out here
    if *splash_screen >= 1 {
        next_screen.set(Screen::Loading);
        return;
    } else {
        // despawn existing, but not if we're about to change scenes because then the SceneScope will do it
        for image_entity in screens.iter() {
            // for some reason I can't fathom this causes a B0003 warning. It seems like this
            // is queued and then not executed for another second or two, its very confusing.
            // it all seems to work so ???
            commands.entity(image_entity).despawn();
        }
    }

    // move to the next splash screen
    *splash_screen = 1;

    // spawn new
    for container in containers.iter() {
        let child = commands
            .spawn(splash_image_bundle(&asset_server, "images/wilsk_logo.png"))
            .id();
        commands.entity(container).add_child(child);
    }

    // reset the timer
    timer.0.reset();
    timer.0.unpause();
}
