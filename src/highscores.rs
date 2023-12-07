use crate::{cleanup, click_sound, GameState};
use bevy::prelude::*;

#[derive(Component)]
struct HighScoresEntity;

const TRANSPARENT: Color = Color::Rgba {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
    alpha: 0.0,
};

pub struct HighScoresPlugin;
impl Plugin for HighScoresPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::HighScores), setup_highscores)
            .add_systems(
                OnExit(GameState::HighScores),
                (click_sound, cleanup::<HighScoresEntity>),
            )
            .add_systems(Update, ui_action.run_if(in_state(GameState::HighScores)));
    }
}

fn setup_highscores(mut commands: Commands, asset_server: Res<AssetServer>) {
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

            parent
                .spawn(ButtonBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        left: Val::Px(10.0),
                        bottom: Val::Px(10.0),
                        ..default()
                    },
                    background_color: TRANSPARENT.into(),
                    ..default()
                })
                .with_children(|parent| {
                    let icon = asset_server.load("sprites/back.png");
                    parent.spawn(ImageBundle {
                        image: UiImage::new(icon),
                        ..default()
                    });
                });
        });
}

fn ui_action(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            game_state.set(GameState::Menu);
        }
    }
}
