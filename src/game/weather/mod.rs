//! Adds weather effects to the game.

use bevy::prelude::*;
use rand::Rng;

use crate::screen::Screen;

use super::{
    animation::PlayerAnimation,
    assets::{HandleMap, ImageKey},
    movement::{
        DespawnWhenOutOfWindow, MoveTowardsLocation, WHALE_TRAVEL_SPEED, WINDOW_DESPAWN_BUFFER,
    },
    spawn::WindowSize,
};

mod day_night_cycle;
pub use day_night_cycle::TintWithDayNightCycle;

#[derive(Event, Debug)]
pub struct SpawnWave {
    x: f32,
    y: f32,
    start_frame: usize,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), spawn_initial_waves);

    app.add_systems(Update, spawn_random_waves.run_if(in_state(Screen::Playing)));

    app.observe(spawn_wave);

    app.add_plugins(day_night_cycle::plugin);
}

fn spawn_initial_waves(mut commands: Commands, win_size: Res<WindowSize>) {
    let mut rng = rand::thread_rng();

    let size = win_size.size();
    let x_range =
        (-size.x / 2.0 - 0.9 * WINDOW_DESPAWN_BUFFER)..(size.x / 2.0 + 0.9 * WINDOW_DESPAWN_BUFFER);
    let y_range = (-size.y / 2.0 + 32.)..(size.y / 2.0 - 32.);

    for _ in 0..30 {
        commands.trigger(SpawnWave {
            x: rng.gen_range(x_range.clone()),
            y: rng.gen_range(y_range.clone()),
            start_frame: rng.gen_range(1..8),
        });
    }
}

fn spawn_random_waves(
    mut commands: Commands,
    time: Res<Time>,
    win_size: Res<WindowSize>,
    mut next_spawn: Local<f32>,
) {
    if *next_spawn > time.elapsed_seconds() {
        return;
    }

    let mut rng = rand::thread_rng();
    *next_spawn = time.elapsed_seconds() + rng.gen_range(0.3..1.2);

    let half_size = win_size.half();

    if half_size.length_squared() < 1. {
        // window is probably minimised? in a real game I guess we could cache the window size
        // so there aren't huge gaps in waves, but whatever
        return;
    }

    let y = -half_size.y - 64.;
    let x = rng.gen_range(
        -half_size.x - 0.5 * WINDOW_DESPAWN_BUFFER..half_size.x + 0.5 * WINDOW_DESPAWN_BUFFER,
    );

    commands.trigger(SpawnWave {
        x,
        y,
        start_frame: 1,
    });
}

fn spawn_wave(
    trigger: Trigger<SpawnWave>,
    mut commands: Commands,
    win_size: Res<WindowSize>,
    image_handles: Res<HandleMap<ImageKey>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let event = trigger.event();

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 9, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let mut player_animation = PlayerAnimation::wave();
    player_animation.set_frame(event.start_frame);

    commands.spawn((
        Name::new("Wave"),
        SpriteBundle {
            texture: image_handles[&ImageKey::Features].clone_weak(),
            transform: Transform::from_xyz(event.x, event.y, 0.)
                .with_scale(Vec3::splat(rand::thread_rng().gen_range(0.98..1.2))),
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: player_animation.get_atlas_index(),
        },
        player_animation,
        MoveTowardsLocation {
            speed: WHALE_TRAVEL_SPEED,
            target: Vec3::new(event.x, win_size.size().y, 0.0),
            remove_on_arrival: false,
        },
        DespawnWhenOutOfWindow,
        StateScoped(Screen::Playing),
    ));
}
