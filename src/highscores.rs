use crate::{cleanup, click_sound, GameState};
use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component)]
struct HighScoresEntity;

pub struct HighScoresPlugin;
impl Plugin for HighScoresPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::HighScores), setup_highscores)
            .add_systems(
                OnExit(GameState::HighScores),
                (click_sound, cleanup::<HighScoresEntity>),
            )
            .add_systems(
                Update,
                (go_back.run_if(in_state(GameState::HighScores).and_then(has_clicked_back)),),
            );
    }
}

fn setup_highscores(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/back.png"),
            transform: Transform::from_xyz(-160.0 + 10.0 + 32.0, -240.0 + 10.0 + 32.0, 100.0),
            ..Default::default()
        },
        HighScoresEntity,
    ));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            HighScoresEntity,
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "HIGHSCORES",
                    TextStyle {
                        font: asset_server.load("fonts/Retroville NC.ttf"),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment::Center),
            );

            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexStart,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    for i in 1..6 {
                        parent.spawn(
                            TextBundle::from_section(
                                format!("{}. {}", i, 100 - (i - 1) * 20),
                                TextStyle {
                                    font: asset_server.load("fonts/Retroville NC.ttf"),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                            )
                            .with_text_alignment(TextAlignment::Left),
                        );
                    }
                });
        });
}

pub fn has_clicked_back(
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button_input: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) -> bool {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Some(position) = q_windows.single().cursor_position() {
            return back_button_pressed(position);
        }
    }

    keyboard_input.just_pressed(KeyCode::Back)
}

fn back_button_pressed(position: Vec2) -> bool {
    position.x > 10.0
        && position.x < 10.0 + 64.0
        && position.y > 480.0 - 10.0 - 64.0
        && position.y < 480.0 - 10.0
}

fn go_back(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Menu);
}
