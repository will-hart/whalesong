//! Adds weather effects to the game. Borrows some day night cycle stuff from here
//! https://github.com/will-hart/bevy_jam_2/blob/main/src/game/day_night_cycle.rs

use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;

use crate::screen::Screen;

use super::{
    animation::PlayerAnimation,
    assets::{HandleMap, ImageKey},
    movement::{DespawnWhenOutOfWindow, MovementController},
};

#[derive(Event, Debug)]
pub struct SpawnWave {
    x: f32,
    y: f32,
}

#[derive(Component)]
pub struct Wave;

pub const WHALE_TRAVEL_SPEED: f32 = 0.3;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), spawn_initial_waves);

    app.add_systems(
        Update,
        (day_night_cycle, spawn_random_waves).run_if(in_state(Screen::Playing)),
    );

    app.add_systems(FixedUpdate, move_waves.run_if(in_state(Screen::Playing)));

    app.observe(spawn_wave);
}

fn spawn_initial_waves(mut commands: Commands, windows: Query<&Window, With<PrimaryWindow>>) {
    let mut rng = rand::thread_rng();

    let size = match windows.get_single() {
        Ok(w) => w.size(),
        Err(_) => return,
    };
    let x_range = (-size.x / 2.0 + 32.)..(size.x / 2.0 - 32.);
    let y_range = (-size.y / 2.0 + 32.)..(size.y / 2.0 - 32.);

    for _ in 0..20 {
        commands.trigger(SpawnWave {
            x: rng.gen_range(x_range.clone()),
            y: rng.gen_range(y_range.clone()),
        });
    }
}

fn spawn_random_waves(
    mut commands: Commands,
    time: Res<Time>,
    mut next_spawn: Local<f32>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    if *next_spawn > time.elapsed_seconds() {
        return;
    }

    let mut rng = rand::thread_rng();
    *next_spawn = time.elapsed_seconds() + rng.gen_range(1.0..3.0);

    let size = match windows.get_single() {
        Ok(w) => w.size(),
        Err(_) => return,
    };

    if size.length_squared() < 1. {
        // window is probably minimised? in a real game I guess we could cache the window size
        // so there aren't huge gaps in waves, but whatever
        return;
    }

    let y = -(size.y / 2.) - 64.;
    let x = rng.gen_range((-size.x / 2.0 + 32.)..(size.x / 2.0 - 32.));

    commands.trigger(SpawnWave { x, y });
}

fn day_night_cycle(mut clear_colour: ResMut<ClearColor>) {
    clear_colour.0 = Color::srgb(0.97, 0.97, 0.85);
}

fn move_waves(movements: Query<&MovementController>, mut waves: Query<&mut Transform, With<Wave>>) {
    let movement = movements.single();

    for mut wave in waves.iter_mut() {
        wave.translation += Vec3::new(
            -movement.0.x * WHALE_TRAVEL_SPEED * 1.5,
            WHALE_TRAVEL_SPEED,
            0.,
        );
    }
}

fn spawn_wave(
    trigger: Trigger<SpawnWave>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 9, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_animation = PlayerAnimation::wave();

    let event = trigger.event();

    commands.spawn((
        Name::new("Player"),
        SpriteBundle {
            texture: image_handles[&ImageKey::Features].clone_weak(),
            transform: Transform::from_xyz(event.x, event.y, 0.)
                .with_scale(Vec3::splat(rand::thread_rng().gen_range(0.98..1.3))),
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: player_animation.get_atlas_index(),
        },
        player_animation,
        Wave,
        DespawnWhenOutOfWindow,
        StateScoped(Screen::Playing),
    ));
}
