use crate::{cleanup, AudioHandles, GameState, SoundDisabled};
use bevy::prelude::*;

mod bob;
mod coin;

#[derive(Component)]
struct GameEntity;

#[derive(Resource, Default)]
pub struct Points(usize);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum PlayState {
    #[default]
    Ready,
    Running,
    Paused,
    LevelEnd,
    GameOver,
}

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<PlayState>()
            .init_resource::<Points>()
            .add_systems(OnEnter(GameState::Playing), setup_play)
            .add_systems(OnExit(GameState::Playing), cleanup::<GameEntity>)
            .add_systems(
                Update,
                (
                    coin_sound.run_if(
                        resource_changed::<Points>().and_then(not(resource_added::<Points>())),
                    ),
                    bob::animate_bob.run_if(in_state(GameState::Playing)),
                    coin::animate_coins.run_if(in_state(GameState::Playing)),
                ),
            );
    }
}

fn setup_play(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    bob::setup_bob(&mut commands, &asset_server, &mut texture_atlases);

    coin::spawn_coin(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        Vec2::new(0.0, 120.0),
    );

    // Spawn the score UI
    commands
        .spawn((
            GameEntity,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|node| {
            node.spawn((TextBundle::from_section(
                "Super Jumper",
                TextStyle {
                    font: asset_server.load("fonts/Retroville NC.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            )
            .with_text_alignment(TextAlignment::Center),));
        });
}

fn coin_sound(
    audio_handles: Res<AudioHandles>,
    mut commands: Commands,
    sound_disabled: Res<SoundDisabled>,
) {
    if !sound_disabled.0 {
        commands.spawn(AudioBundle {
            source: audio_handles.coin.clone(),
            ..default()
        });
    }
}
