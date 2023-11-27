use crate::{cleanup, click_sound, GameState};
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct HelpScreenIndex(usize);

pub struct HelpPlugin;
impl Plugin for HelpPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HelpScreenIndex>()
            .add_systems(OnEnter(GameState::Help), setup_help)
            .add_systems(OnExit(GameState::Help), cleanup::<HelpEntity>)
            .add_systems(
                Update,
                (
                    show_next_screen.run_if(in_state(GameState::Help).and_then(has_user_input)),
                    click_sound.run_if(
                        resource_changed::<HelpScreenIndex>()
                            .and_then(not(resource_added::<HelpScreenIndex>())),
                    ),
                ),
            );
    }
}

#[derive(Component)]
struct HelpEntity;

fn setup_help(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/help1.png"),
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            ..Default::default()
        },
        HelpEntity,
    ));
}

fn show_next_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut help_screen: ResMut<HelpScreenIndex>,
    mut state: ResMut<NextState<GameState>>,
) {
    help_screen.0 += 1;
    if help_screen.0 < 5 {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load(format!("sprites/help{}.png", help_screen.0 + 1)),
                transform: Transform::from_xyz(0.0, 0.0, 100.0 + (help_screen.0 as f32)),
                ..Default::default()
            },
            HelpEntity,
        ));
    } else {
        help_screen.0 = 0;
        state.set(GameState::Menu);
    }
}

pub fn has_user_input(
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button_input: Res<Input<MouseButton>>,
    touch_input: Res<Touches>,
) -> bool {
    keyboard_input.just_pressed(KeyCode::Space)
        || mouse_button_input.just_pressed(MouseButton::Left)
        || touch_input.any_just_pressed()
}
