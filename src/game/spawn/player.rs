//! Spawn the player.

use bevoids::boids::BoidRepulsor;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    game::{
        animation::PlayerAnimation,
        assets::{HandleMap, ImageKey},
        movement::{Movement, MovementController, WHALE_TRAVEL_SPEED},
    },
    screen::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_player).add_systems(
        FixedUpdate,
        move_in_spawning_whale.run_if(in_state(Screen::Playing)),
    );
    app.register_type::<Whale>();
    app.init_resource::<WhaleLocation>();
}

#[derive(Event, Debug)]
pub struct SpawnPlayer;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Whale;

#[derive(Resource, Default)]
pub struct WhaleLocation {
    pub y: f32,
    pub current_rotation: f32,
    pub target_rotation: f32,
}

#[derive(Component)]
pub struct InputHelp;

#[derive(Component)]
pub struct WhaleArrivalMarker;

fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    mut whale_pos: ResMut<WhaleLocation>,
    image_handles: Res<HandleMap<ImageKey>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 8, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_animation = PlayerAnimation::new();

    let height = match windows.get_single() {
        Ok(w) => w.height(),
        Err(_) => return,
    };
    let half_height = height / 2.0;
    whale_pos.y = half_height * 0.5;

    let start_pos = Vec3::new(0.0, half_height + 64., 0.);

    commands
        .spawn((
            Name::new("Player"),
            Whale,
            BoidRepulsor {
                strength: 10.,
                range: 80.,
            },
            SpriteBundle {
                texture: image_handles[&ImageKey::Creatures].clone_weak(),
                transform: Transform::from_translation(start_pos),
                ..Default::default()
            },
            TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: player_animation.get_atlas_index(),
            },
            MovementController::default(),
            Movement { speed: 420.0 },
            player_animation,
            WhaleArrivalMarker,
            StateScoped(Screen::Playing),
        ))
        .with_children(|parent| {
            let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 8, 1, None, None);
            let texture_atlas_layout = texture_atlas_layouts.add(layout);

            parent.spawn((
                Name::new("PlayerHelp Left"),
                InputHelp,
                SpriteBundle {
                    texture: image_handles[&ImageKey::Icons].clone_weak(),
                    transform: Transform::from_scale(Vec3::splat(0.5))
                        .with_translation(Vec3::new(-45., -30., 0.0)),
                    ..Default::default()
                },
                TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: 0,
                },
                StateScoped(Screen::Playing),
            ));
            parent.spawn((
                Name::new("PlayerHelp Right"),
                InputHelp,
                SpriteBundle {
                    texture: image_handles[&ImageKey::Icons].clone_weak(),
                    transform: Transform::from_scale(Vec3::splat(0.5))
                        .with_translation(Vec3::new(45., -30., 0.0)),
                    ..Default::default()
                },
                TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: 1,
                },
                StateScoped(Screen::Playing),
            ));
        });
}

fn move_in_spawning_whale(
    mut commands: Commands,
    whale_pos: Res<WhaleLocation>,
    mut whales: Query<(Entity, &mut Transform), With<WhaleArrivalMarker>>,
) {
    for (entity, mut whale) in &mut whales {
        whale.translation.y -= WHALE_TRAVEL_SPEED;

        if whale.translation.y < whale_pos.y {
            info!("Whale has arrived");
            commands.entity(entity).remove::<WhaleArrivalMarker>();
        }
    }
}
