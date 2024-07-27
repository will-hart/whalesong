use bevy::prelude::*;
use rand::Rng;

use crate::{
    game::{
        animation::AnimationPlayer,
        assets::{HandleMap, ImageKey},
        flipper::Flippable,
        movement::{
            DespawnWhenOutOfWindow, MoveWithVelocity, WHALE_TRAVEL_SPEED, WINDOW_DESPAWN_BUFFER,
        },
        spawn::WindowSize,
    },
    screen::Screen,
};

use super::TravelDistance;

/// Marks waves as .... waves
#[derive(Component)]
pub struct Wave;

#[derive(Event, Debug)]
pub struct SpawnWave {
    x: f32,
    y: f32,
    start_frame: usize,
    with_velocity: bool,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), spawn_initial_waves);
    app.add_systems(Update, spawn_random_waves.run_if(in_state(Screen::Playing)));
    app.observe(spawn_wave);
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
            with_velocity: false,
        });
    }
}

fn spawn_random_waves(
    mut commands: Commands,
    distance: Res<TravelDistance>,
    win_size: Res<WindowSize>,
    mut next_spawn: Local<f32>,
) {
    if distance.get() < 1. {
        *next_spawn = 3.;
    }

    if *next_spawn > distance.get() {
        return;
    }

    *next_spawn = distance.future_range(0.25..1.1);

    let half_size = win_size.half();
    if half_size.length_squared() < 1. {
        // window is probably minimised? in a real game I guess we could cache the window size
        // so there aren't huge gaps in waves, but whatever
        return;
    }

    let mut rng = rand::thread_rng();

    let y = -half_size.y - 64.;
    let x = rng.gen_range(
        -half_size.x - 0.5 * WINDOW_DESPAWN_BUFFER..half_size.x + 0.5 * WINDOW_DESPAWN_BUFFER,
    );

    commands.trigger(SpawnWave {
        x,
        y,
        start_frame: 1,
        with_velocity: true,
    });
}

fn spawn_wave(
    trigger: Trigger<SpawnWave>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let event = trigger.event();

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 9, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let mut player_animation = AnimationPlayer::wave();
    player_animation.set_frame(event.start_frame);

    let mut cmds = commands.spawn((
        Name::new("Wave"),
        SpriteBundle {
            texture: image_handles[&ImageKey::Features].clone_weak(),
            transform: Transform::from_translation(Vec3::new(event.x, event.y, 0.0))
                .with_scale(Vec3::splat(rand::thread_rng().gen_range(0.98..1.2))),
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: player_animation.get_atlas_index(),
        },
        Wave,
        player_animation,
        Flippable,
        DespawnWhenOutOfWindow,
        StateScoped(Screen::Playing),
    ));

    if event.with_velocity {
        cmds.insert(MoveWithVelocity(Vec3::Y * WHALE_TRAVEL_SPEED));
    }
}
