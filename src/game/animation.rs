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

pub const WHALE_BREATH_FRAME_RATE: u64 = 150;

#[derive(Event)]
pub struct AnimationComplete;

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
fn update_animation_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut PlayerAnimation)>,
) {
    for (entity, mut animation) in &mut query {
        if animation.update_timer(time.delta()) {
            commands.trigger_targets(AnimationComplete, entity)
        }
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
#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct PlayerAnimation {
    timer: Timer,
    frame: usize,
    state: PlayerAnimationState,
    oneshot: bool,
}

#[derive(Reflect, PartialEq, Clone)]
pub enum PlayerAnimationState {
    WhaleSwimming,
    Wave,
    Bird,
    Fish,
    WhaleBreath,
}

impl PlayerAnimation {
    /// The duration of each idle frame.
    const IDLE_INTERVAL: Duration = Duration::from_millis(250);

    fn swimming() -> Self {
        Self {
            timer: Timer::new(Self::IDLE_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::WhaleSwimming,
            oneshot: false,
        }
    }

    pub fn wave() -> Self {
        Self {
            timer: Timer::new(Duration::from_millis(450), TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Wave,
            oneshot: false,
        }
    }

    pub fn bird() -> Self {
        Self {
            timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Bird,
            oneshot: false,
        }
    }

    pub fn fish() -> Self {
        Self {
            timer: Timer::new(Duration::from_millis(200), TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Fish,
            oneshot: false,
        }
    }

    pub fn breath() -> Self {
        Self {
            timer: Timer::new(
                Duration::from_millis(WHALE_BREATH_FRAME_RATE),
                TimerMode::Repeating,
            ),
            frame: 0,
            state: PlayerAnimationState::WhaleBreath,
            oneshot: true,
        }
    }

    pub fn new() -> Self {
        Self::swimming()
    }

    pub fn set_frame(&mut self, frame: usize) {
        // don't do wrapping here as that happens in `update_timer`, just assume we got it right
        self.frame = frame;
    }

    /// Update animation timers. Returns true if
    /// - the animation ticked,
    /// - this is a oneshot animation, and
    /// - the animation index has wrapped back to the start.
    /// For now ignore that some animations don't start at the lowest index frame.
    pub fn update_timer(&mut self, delta: Duration) -> bool {
        self.timer.tick(delta);
        if !self.timer.finished() {
            return false;
        }

        let prev = self.frame;
        self.frame = (self.frame + 1)
            % match self.state {
                PlayerAnimationState::Wave => 9,
                PlayerAnimationState::WhaleSwimming
                | PlayerAnimationState::Bird
                | PlayerAnimationState::Fish => 8,
                PlayerAnimationState::WhaleBreath => 16,
            };

        self.oneshot && self.frame < prev
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
            PlayerAnimationState::Bird => BIRD_START_FRAME + self.frame,
            PlayerAnimationState::WhaleSwimming
            | PlayerAnimationState::Wave
            | PlayerAnimationState::Fish => self.frame,
            PlayerAnimationState::WhaleBreath => 8 + self.frame,
        }
    }
}
