//! Player sprite animation.
//! This is based on multiple examples and may be very different for your game.
//! - [Sprite flipping](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_flipping.rs)
//! - [Sprite animation](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_animation.rs)
//! - [Timers](https://github.com/bevyengine/bevy/blob/latest/examples/time/timers.rs)

use std::time::Duration;

use bevy::prelude::*;

use crate::AppSet;

/// The frame number where the whale starts to turn
const BIRD_START_FRAME: usize = 32;

pub(super) fn plugin(app: &mut App) {
    // Animate and play sound effects based on controls.
    app.register_type::<PlayerAnimation>();
    app.add_systems(
        Update,
        (
            update_animation_timer.in_set(AppSet::TickTimers),
            update_animation_atlas.in_set(AppSet::Update),
        ),
    );
}

/// Update the animation timer.
fn update_animation_timer(time: Res<Time>, mut query: Query<&mut PlayerAnimation>) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}

/// Update the texture atlas to reflect changes in the animation.
fn update_animation_atlas(mut query: Query<(&PlayerAnimation, &mut TextureAtlas)>) {
    for (animation, mut atlas) in &mut query {
        if animation.changed() {
            atlas.index = animation.get_atlas_index();
        }
    }
}

/// Component that tracks player's animation state.
/// It is tightly bound to the texture atlas we use.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerAnimation {
    timer: Timer,
    frame: usize,
    state: PlayerAnimationState,
}

#[derive(Reflect, PartialEq)]
pub enum PlayerAnimationState {
    WhaleSwimming,
    Wave,
    Bird,
    Fish,
}

impl PlayerAnimation {
    /// The duration of each idle frame.
    const IDLE_INTERVAL: Duration = Duration::from_millis(250);

    fn swimming() -> Self {
        Self {
            timer: Timer::new(Self::IDLE_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::WhaleSwimming,
        }
    }

    pub fn wave() -> Self {
        Self {
            timer: Timer::new(Duration::from_millis(450), TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Wave,
        }
    }

    pub fn bird() -> Self {
        Self {
            timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Bird,
        }
    }

    pub fn fish() -> Self {
        Self {
            timer: Timer::new(Duration::from_millis(200), TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Fish,
        }
    }

    pub fn new() -> Self {
        Self::swimming()
    }

    pub fn set_frame(&mut self, frame: usize) {
        // don't do wrapping here as that happens in `update_timer`, just assume we got it right
        self.frame = frame;
    }

    /// Update animation timers.
    pub fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if !self.timer.finished() {
            return;
        }
        self.frame = (self.frame + 1)
            % match self.state {
                PlayerAnimationState::Wave => 9,
                PlayerAnimationState::WhaleSwimming
                | PlayerAnimationState::Bird
                | PlayerAnimationState::Fish => 8,
            };
    }

    // Leaving here in case I have time to add different animations
    // /// Update animation state if it changes.
    // pub fn update_state(&mut self, state: PlayerAnimationState) {
    //     if self.state != state {
    //         match state {
    //             PlayerAnimationState::WhaleSwimming => *self = Self::swimming(),
    //             PlayerAnimationState::Wave => *self = Self::wave(),
    //             PlayerAnimationState::Bird => *self = Self::bird(),
    //             PlayerAnimationState::Fish => *self = Self::fish(),
    //         }
    //     }
    // }

    /// Whether animation changed this tick.
    pub fn changed(&self) -> bool {
        self.timer.finished()
    }

    /// Return sprite index in the atlas.
    pub fn get_atlas_index(&self) -> usize {
        match self.state {
            PlayerAnimationState::WhaleSwimming => self.frame,
            PlayerAnimationState::Wave => self.frame,
            PlayerAnimationState::Bird => BIRD_START_FRAME + self.frame,
            PlayerAnimationState::Fish => self.frame,
        }
    }
}
