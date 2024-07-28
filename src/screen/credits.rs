//! A credits screen that can be accessed from the title screen.

use bevy::prelude::*;

use super::Screen;
use crate::{
    game::assets::{HandleMap, ImageKey},
    ui::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Credits), enter_credits);

    app.add_systems(
        Update,
        handle_credits_action.run_if(in_state(Screen::Credits)),
    );
    app.register_type::<CreditsAction>();
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum CreditsAction {
    Back,
}

fn enter_credits(mut commands: Commands, image_handles: Res<HandleMap<ImageKey>>) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Credits))
        .with_children(|children| {
            children.spawn(ImageBundle {
                image: image_handles[&ImageKey::Credits].clone_weak().into(),
                ..Default::default()
            });
        });
}

fn handle_credits_action(
    mut next_screen: ResMut<NextState<Screen>>,
    input: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if input.any_just_pressed([KeyCode::Escape, KeyCode::Space])
        || mouse.just_pressed(MouseButton::Left)
        || mouse.just_pressed(MouseButton::Right)
    {
        next_screen.set(Screen::Title);
    }
}
