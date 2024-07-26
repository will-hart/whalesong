use bevy::{audio::PlaybackMode, prelude::*};

use crate::game::assets::{HandleMap, SfxKey};

pub(super) fn plugin(app: &mut App) {
    app.observe(play_sfx);
}

fn play_sfx(
    trigger: Trigger<PlaySfx>,
    mut commands: Commands,
    sfx_handles: Res<HandleMap<SfxKey>>,
) {
    let request = trigger.event();
    let sfx_key = request.key;

    let bundle = AudioSourceBundle {
        source: sfx_handles[&sfx_key].clone_weak(),
        settings: PlaybackSettings {
            mode: if request.looped {
                PlaybackMode::Loop
            } else {
                PlaybackMode::Despawn
            },
            ..default()
        },
    };

    if let Some(parent) = request.parent_entity {
        commands.entity(parent).with_children(|child| {
            child.spawn(bundle);
        });
    } else {
        commands.spawn(bundle);
    }
}

/// Trigger this event to play a single sound effect.
#[derive(Event)]
pub struct PlaySfx {
    pub key: SfxKey,
    pub looped: bool,
    pub parent_entity: Option<Entity>,
}

impl PlaySfx {
    pub fn once(key: SfxKey) -> Self {
        Self {
            key,
            looped: false,
            parent_entity: None,
        }
    }

    pub fn looped(key: SfxKey) -> Self {
        Self {
            key,
            looped: true,
            parent_entity: None,
        }
    }

    pub fn with_parent(mut self, entity: Entity) -> Self {
        self.parent_entity = Some(entity);
        self
    }
}
