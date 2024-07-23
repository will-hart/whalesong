//! Spawn the player.

use bevy::prelude::*;
use rand::Rng;

use crate::{
    game::{
        animation::PlayerAnimation,
        assets::{HandleMap, ImageKey},
        movement::{MoveTowardsLocation, MoveWithWhale, WHALE_TRAVEL_SPEED},
    },
    screen::Screen,
};

use super::WindowSize;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_creature);
    app.add_systems(OnEnter(Screen::Playing), spawn_random_creatures);
}

fn spawn_random_creatures(mut commands: Commands) {
    commands.trigger(SpawnCreature {
        creature_type: CreatureType::Bird,
    });
}

#[derive(Event, Debug)]
pub struct SpawnCreature {
    pub creature_type: CreatureType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CreatureType {
    #[default]
    Bird,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Creature(pub CreatureType);

fn spawn_creature(
    _trigger: Trigger<SpawnCreature>,
    mut commands: Commands,
    win_size: Res<WindowSize>,
    image_handles: Res<HandleMap<ImageKey>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 8, 3, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_animation = match _trigger.event().creature_type {
        CreatureType::Bird => PlayerAnimation::bird(),
    };

    let size = win_size.size();
    let (from_pos, to_pos) = get_creature_path(size, 64.);

    commands.spawn((
        Name::new("Bird"),
        Creature(CreatureType::Bird),
        SpriteBundle {
            texture: image_handles[&ImageKey::Creatures].clone_weak(),
            transform: Transform::from_translation(from_pos),
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: player_animation.get_atlas_index(),
        },
        player_animation,
        StateScoped(Screen::Playing),
        MoveTowardsLocation {
            speed: WHALE_TRAVEL_SPEED * 1.2,
            target: to_pos,
        },
        MoveWithWhale,
    ));
}

/// Returns the ends of a path for spawning a creature
fn get_creature_path(window_size: Vec2, sprite_size: f32) -> (Vec3, Vec3) {
    let mut rng = rand::thread_rng();
    let half_size = window_size / 2.0;

    // check if we're going from L-R or U-D
    let (a, b) = if rng.gen_bool(0.5) {
        // L-R
        (
            Vec2::new(
                -half_size.x - sprite_size,
                rng.gen_range(-half_size.y + sprite_size..half_size.y - sprite_size),
            ),
            Vec2::new(
                half_size.x + sprite_size,
                rng.gen_range(-half_size.y + sprite_size..half_size.y - sprite_size),
            ),
        )
    } else {
        // U-D
        (
            Vec2::new(
                rng.gen_range(-half_size.x + sprite_size..half_size.x - sprite_size),
                -half_size.y - sprite_size,
            ),
            Vec2::new(
                rng.gen_range(-half_size.x + sprite_size..half_size.x - sprite_size),
                half_size.y + sprite_size,
            ),
        )
    };

    // check if we should switch them
    if rng.gen_bool(0.5) {
        (a.extend(0.), b.extend(0.))
    } else {
        (b.extend(0.), a.extend(0.))
    }
}
