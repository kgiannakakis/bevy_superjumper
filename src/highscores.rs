use crate::{
    GameState, cleanup, click_sound,
    settings::{HIGHSCORE_COUNT, read_settings, write_high_scores},
};
use bevy::prelude::*;

#[derive(Component)]
struct HighScoresEntity;

#[derive(Resource)]
pub struct HighScores([u32; HIGHSCORE_COUNT]);

impl Default for HighScores {
    fn default() -> Self {
        Self(read_settings().high_scores)
    }
}

const TRANSPARENT: Color = Color::linear_rgba(0.0, 0.0, 0.0, 0.0);

pub struct HighScoresPlugin;
impl Plugin for HighScoresPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HighScores>()
            .add_systems(OnEnter(GameState::HighScores), setup_highscores)
            .add_systems(
                OnExit(GameState::HighScores),
                (click_sound, cleanup::<HighScoresEntity>),
            )
            .add_systems(Update, ui_action.run_if(in_state(GameState::HighScores)));
    }
}

fn setup_highscores(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    high_scores: Res<HighScores>,
) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            HighScoresEntity,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("HIGHSCORES"),
                TextFont {
                    font: asset_server.load("fonts/Retroville NC.ttf"),
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(Justify::Center),
            ));

            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexStart,
                    ..default()
                })
                .with_children(|parent| {
                    for (i, score) in high_scores.0.into_iter().enumerate() {
                        parent.spawn((
                            Text::new(format!("{}. {}", i + 1, score)),
                            TextFont {
                                font: asset_server.load("fonts/Retroville NC.ttf"),
                                font_size: 30.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                            TextLayout::new_with_justify(Justify::Center),
                        ));
                    }
                });

            parent
                .spawn((
                    Button,
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(10.0),
                        bottom: Val::Px(10.0),
                        width: Val::Px(64.0),
                        height: Val::Px(64.0),

                        ..default()
                    },
                    BackgroundColor(TRANSPARENT),
                ))
                .with_children(|parent| {
                    let icon = asset_server.load("sprites/back.png");
                    parent.spawn((ImageNode::new(icon),));
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

pub fn check_and_update_highscores(high_scores: &mut ResMut<HighScores>, score: u32) -> bool {
    let mut is_highscore = false;
    let mut prev_score: u32 = 0;
    for (i, current_score) in high_scores.0.into_iter().enumerate() {
        if is_highscore {
            std::mem::swap(&mut high_scores.0[i], &mut prev_score);
        } else if current_score < score {
            is_highscore = true;
            prev_score = current_score;
            high_scores.0[i] = score;
        }
    }
    if is_highscore {
        write_high_scores(high_scores.0);
    }
    is_highscore
}
