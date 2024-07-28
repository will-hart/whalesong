//! Player sprite animation.
//! This is based on multiple examples and may be very different for your game.
//! - [Sprite flipping](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_flipping.rs)
//! - [Sprite animation](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_animation.rs)
//! - [Timers](https://github.com/bevyengine/bevy/blob/latest/examples/time/timers.rs)

use std::time::Duration;

use bevy::prelude::*;

use crate::AppSet;

/// The frame number where the whale starts to turn
const BIRD_START_FRAME: usize = 48;

pub const WHALE_BREATH_FRAME_RATE: u64 = 150;

#[derive(Event)]
pub struct AnimationComplete(pub AnimationPlayerState);

/// A shared observer that can be added to despawn the element when the animation is complete
pub fn despawn_when_animation_complete(
    trigger: Trigger<AnimationComplete>,
    mut commands: Commands,
) {
    commands.entity(trigger.entity()).despawn();
}

pub(super) fn plugin(app: &mut App) {
    // Animate and play sound effects based on controls.
    app.register_type::<SpriteAnimationPlayer>();
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
    mut query: Query<(Entity, &mut SpriteAnimationPlayer)>,
) {
    for (entity, mut animation) in &mut query {
        if animation.update_timer(time.delta()) {
            commands.trigger_targets(AnimationComplete(animation.state), entity)
        }
    }
}

/// Update the texture atlas to reflect changes in the animation.
fn update_animation_atlas(mut query: Query<(&SpriteAnimationPlayer, &mut TextureAtlas)>) {
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
pub struct SpriteAnimationPlayer {
    timer: Timer,
    frame: usize,
    state: AnimationPlayerState,
    oneshot: bool,
}

#[derive(Reflect, PartialEq, Clone, Copy, Debug)]
pub enum AnimationPlayerState {
    WhaleSwimming,
    BabyWhaleSwimming,
    WhaleBreaching,
    Wave,
    Bird,
    Ship,
    Fish,
    WhaleBreath,
    RainDrop,
}

pub const WHALE_FRAME_MILLIS: u64 = 230;
pub const FAST_WHALE_FRAME_MILLIS: u64 = 180;
pub const SLOW_WHALE_FRAME_MILLIS: u64 = 290;

impl SpriteAnimationPlayer {
    /// The duration of each idle frame.
    const SWIM_INTERVAL: Duration = Duration::from_millis(WHALE_FRAME_MILLIS);
    const BABY_SWIM_INTERVAL: Duration = Duration::from_millis(FAST_WHALE_FRAME_MILLIS);
    const BREACH_INTERVAL: Duration = Duration::from_millis(FAST_WHALE_FRAME_MILLIS);

    fn swimming() -> Self {
        Self {
            timer: Timer::new(Self::SWIM_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: AnimationPlayerState::WhaleSwimming,
            oneshot: false,
        }
    }

    pub fn baby_swimming() -> Self {
        Self {
            timer: Timer::new(Self::BABY_SWIM_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: AnimationPlayerState::BabyWhaleSwimming,
            oneshot: false,
        }
    }

    fn breaching() -> Self {
        Self {
            timer: Timer::new(Self::BREACH_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: AnimationPlayerState::WhaleBreaching,
            oneshot: true,
        }
    }

    pub fn wave() -> Self {
        Self {
            timer: Timer::new(Duration::from_millis(450), TimerMode::Repeating),
            frame: 0,
            state: AnimationPlayerState::Wave,
            oneshot: false,
        }
    }

    pub fn bird() -> Self {
        Self {
            timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
            frame: 0,
            state: AnimationPlayerState::Bird,
            oneshot: false,
        }
    }

    pub fn ship() -> Self {
        Self {
            timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
            frame: 0,
            state: AnimationPlayerState::Ship,
            oneshot: false,
        }
    }

    pub fn raindrop() -> Self {
        Self {
            timer: Timer::new(Duration::from_millis(100), TimerMode::Repeating),
            frame: 0,
            state: AnimationPlayerState::RainDrop,
            oneshot: true,
        }
    }

    pub fn fish() -> Self {
        Self {
            timer: Timer::new(Duration::from_millis(200), TimerMode::Repeating),
            frame: 0,
            state: AnimationPlayerState::Fish,
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
            state: AnimationPlayerState::WhaleBreath,
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

    /// Changes the interval on the frame timer, retaining mode and elapsed time since the last tick
    pub fn set_frame_interval(&mut self, interval: u64) {
        // self.timer.reset();
        self.timer.set_duration(Duration::from_millis(interval));
        self.timer.unpause();
    }

    /// Update animation timers. Returns true if
    /// - the animation ticked,
    /// - this is a oneshot animation, and
    /// - the animation index has wrapped back to the start.
    ///
    /// For now ignore that some animations don't start at the lowest index frame.
    pub fn update_timer(&mut self, delta: Duration) -> bool {
        self.timer.tick(delta);
        if !self.timer.finished() {
            return false;
        }

        let prev = self.frame;
        self.frame = (self.frame + 1)
            % match self.state {
                AnimationPlayerState::Ship => 4,
                AnimationPlayerState::WhaleSwimming
                | AnimationPlayerState::Bird
                | AnimationPlayerState::Fish
                | AnimationPlayerState::RainDrop
                | AnimationPlayerState::BabyWhaleSwimming => 8,
                AnimationPlayerState::Wave => 9,
                AnimationPlayerState::WhaleBreath => 16,
                AnimationPlayerState::WhaleBreaching => 24,
            };

        self.oneshot && self.frame < prev
    }

    /// Update animation state if it changes.
    pub fn update_state(&mut self, state: AnimationPlayerState) {
        if self.state != state {
            match state {
                AnimationPlayerState::WhaleSwimming => *self = Self::swimming(),
                AnimationPlayerState::WhaleBreaching => *self = Self::breaching(),
                d => {
                    warn!("Attempted to change to invalid state: {d:?}. This has no effect");
                }
            }
        }
    }

    /// Check whether the animation player is currently in the given state
    pub fn in_state(&self, state: AnimationPlayerState) -> bool {
        self.state == state
    }

    /// Whether animation changed this tick.
    pub fn changed(&self) -> bool {
        self.timer.finished()
    }

    /// Return sprite index in the atlas.
    pub fn get_atlas_index(&self) -> usize {
        match self.state {
            AnimationPlayerState::Ship
            | AnimationPlayerState::WhaleSwimming
            | AnimationPlayerState::Wave
            | AnimationPlayerState::Fish
            | AnimationPlayerState::RainDrop => self.frame,
            AnimationPlayerState::Bird => BIRD_START_FRAME + self.frame,
            AnimationPlayerState::WhaleBreath => 8 + self.frame,
            AnimationPlayerState::WhaleBreaching => 24 + self.frame,
            AnimationPlayerState::BabyWhaleSwimming => 56 + self.frame,
        }
    }
}
