//! Spawn the player.

use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    game::{
        animation::PlayerAnimation,
        assets::{HandleMap, ImageKey},
        movement::{MoveToY, Movement, MovementController},
    },
    screen::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_player);
    app.register_type::<Player>();
}

#[derive(Event, Debug)]
pub struct SpawnPlayer;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerHelp;

fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    // A texture atlas is a way to split one image with a grid into multiple sprites.
    // By attaching it to a [`SpriteBundle`] and providing an index, we can specify which section of the image we want to see.
    // We will use this to animate our player character. You can learn more about texture atlases in this example:
    // https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 8, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_animation = PlayerAnimation::new();

    let height = windows.single().height();
    let half_height = height / 2.0;

    commands
        .spawn((
            Name::new("Player"),
            Player,
            SpriteBundle {
                texture: image_handles[&ImageKey::Creatures].clone_weak(),
                transform: Transform::from_xyz(0.0, half_height + 96., 0.),
                ..Default::default()
            },
            TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: player_animation.get_atlas_index(),
            },
            MovementController::default(),
            Movement { speed: 420.0 },
            MoveToY {
                y: half_height * 0.67,
                speed: 150.,
            },
            player_animation,
            StateScoped(Screen::Playing),
        ))
        .with_children(|parent| {
            let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 8, 1, None, None);
            let texture_atlas_layout = texture_atlas_layouts.add(layout);

            parent.spawn((
                Name::new("PlayerHelp Left"),
                PlayerHelp,
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
                PlayerHelp,
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
