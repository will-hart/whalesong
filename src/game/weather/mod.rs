//! Adds weather effects to the game. Borrows some day night cycle stuff from here
//! https://github.com/will-hart/bevy_jam_2/blob/main/src/game/day_night_cycle.rs

use bevy::prelude::*;
use rand::Rng;

use crate::{screen::Screen, ui::palette::NODE_BACKGROUND};

use super::{
    animation::PlayerAnimation,
    assets::{HandleMap, ImageKey},
    movement::{DespawnWhenOutOfWindow, MoveTowardsLocation, MovesWithWhale, WHALE_TRAVEL_SPEED},
    spawn::WindowSize,
};

#[derive(Event, Debug)]
pub struct SpawnWave {
    x: f32,
    y: f32,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), spawn_initial_waves);

    app.add_systems(
        Update,
        (day_night_cycle, spawn_random_waves).run_if(in_state(Screen::Playing)),
    );

    app.observe(spawn_wave);
}

fn spawn_initial_waves(mut commands: Commands, win_size: Res<WindowSize>) {
    let mut rng = rand::thread_rng();

    let size = win_size.size();
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
    win_size: Res<WindowSize>,
    mut next_spawn: Local<f32>,
) {
    if *next_spawn > time.elapsed_seconds() {
        return;
    }

    let mut rng = rand::thread_rng();
    *next_spawn = time.elapsed_seconds() + rng.gen_range(0.5..2.0);

    let size = win_size.size();

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
    clear_colour.0 = NODE_BACKGROUND;
}

fn spawn_wave(
    trigger: Trigger<SpawnWave>,
    mut commands: Commands,
    win_size: Res<WindowSize>,
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
                .with_scale(Vec3::splat(rand::thread_rng().gen_range(0.3..0.9))),
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
        },
        MovesWithWhale,
        DespawnWhenOutOfWindow,
        StateScoped(Screen::Playing),
    ));
}
