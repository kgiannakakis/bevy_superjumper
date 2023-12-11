use crate::{cleanup, click_sound, GameState};
use bevy::prelude::*;

#[derive(Component)]
struct WinScreenEntity;

#[derive(Resource, Default)]
pub struct WinScreenIndex(usize);

pub struct WinScreenPlugin;
impl Plugin for WinScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WinScreenIndex>()
            .add_systems(OnEnter(GameState::WinScreen), setup_winscreen)
            .add_systems(OnExit(GameState::WinScreen), cleanup::<WinScreenEntity>)
            .add_systems(
                Update,
                (
                    show_next_screen
                        .run_if(in_state(GameState::WinScreen).and_then(has_user_input)),
                    click_sound.run_if(
                        resource_changed::<WinScreenIndex>()
                            .and_then(not(resource_added::<WinScreenIndex>())),
                    ),
                ),
            );
    }
}

fn setup_winscreen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/help1.png"),
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            ..Default::default()
        },
        WinScreenEntity,
    ));
}

fn show_next_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut win_screen: ResMut<WinScreenIndex>,
    mut state: ResMut<NextState<GameState>>,
) {
    win_screen.0 += 1;
    if win_screen.0 < 5 {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load(format!("sprites/help{}.png", win_screen.0 + 1)),
                transform: Transform::from_xyz(0.0, 0.0, 100.0 + (win_screen.0 as f32)),
                ..Default::default()
            },
            WinScreenEntity,
        ));
    } else {
        win_screen.0 = 0;
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
