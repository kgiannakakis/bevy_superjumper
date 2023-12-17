#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{audio::Volume, prelude::*, render::camera::ScalingMode, window::WindowResolution};
use settings::read_settings;

mod game;
mod help;
mod highscores;
mod menu;
mod settings;
mod winscreen;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Playing,
    WinScreen,
    Help,
    HighScores,
}

#[derive(Component)]
struct GameMusic;

#[derive(Component)]
struct Background;

#[derive(Resource)]
struct AudioHandles {
    click: Handle<AudioSource>,
    coin: Handle<AudioSource>,
    jump: Handle<AudioSource>,
    highjump: Handle<AudioSource>,
    hit: Handle<AudioSource>,
}

#[derive(Event, Default)]
enum SoundEvent {
    #[default]
    Click,
    Coin,
    Jump,
    Highjump,
    Hit,
}

#[derive(Resource)]
pub struct SoundEnabled(bool);

impl Default for SoundEnabled {
    fn default() -> Self {
        Self(read_settings().sound_enabled)
    }
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Super Jumper"),
                        resolution: WindowResolution::new(400.0, 600.0),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .init_resource::<SoundEnabled>()
        .add_state::<GameState>()
        .add_event::<SoundEvent>()
        .add_systems(Startup, (scene_setup, play_music))
        .add_systems(Update, handle_sound_event)
        .add_plugins((
            bevy::diagnostic::LogDiagnosticsPlugin::default(),
            bevy::diagnostic::FrameTimeDiagnosticsPlugin,
            menu::MenuPlugin,
            help::HelpPlugin,
            game::GamePlugin,
            highscores::HighScoresPlugin,
            winscreen::WinScreenPlugin,
        ))
        .run();
}

fn scene_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn a 2D camera
    let mut game_2d_camera_bundle = Camera2dBundle::default();
    game_2d_camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(480.0);
    game_2d_camera_bundle.transform = Transform::from_xyz(0.0, 0.0, 0.0);
    commands.spawn(game_2d_camera_bundle);

    // Spawn the background sprite
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/background.png"),
            ..Default::default()
        },
        Background,
    ));

    // Load audio files
    commands.insert_resource(AudioHandles {
        coin: asset_server.load("audio/coin.ogg"),
        hit: asset_server.load("audio/hit.ogg"),
        jump: asset_server.load("audio/jump.ogg"),
        highjump: asset_server.load("audio/highjump.ogg"),
        click: asset_server.load("audio/click.ogg"),
    });
}

// Despawn all entities recursively with a given component
pub fn cleanup<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn play_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sound_enabled: Res<SoundEnabled>,
) {
    if sound_enabled.0 {
        commands.spawn((
            AudioBundle {
                source: asset_server.load("audio/music.ogg"),
                settings: PlaybackSettings::LOOP.with_volume(Volume::new_relative(0.1)),
            },
            GameMusic,
        ));
    }
}

fn handle_sound_event(
    mut commands: Commands,
    audio_handles: Res<AudioHandles>,
    mut sound_events: EventReader<SoundEvent>,
    sound_enabled: Res<SoundEnabled>,
) {
    if !sound_events.is_empty() {
        if sound_enabled.0 {
            for sound_event in sound_events.read() {
                let source = match sound_event {
                    SoundEvent::Click => audio_handles.click.clone(),
                    SoundEvent::Coin => audio_handles.coin.clone(),
                    SoundEvent::Jump => audio_handles.jump.clone(),
                    SoundEvent::Highjump => audio_handles.highjump.clone(),
                    SoundEvent::Hit => audio_handles.hit.clone(),
                };
                commands.spawn(AudioBundle {
                    source,
                    ..default()
                });
            }
        }
        sound_events.clear();
    }
}

fn click_sound(
    audio_handles: Res<AudioHandles>,
    mut commands: Commands,
    sound_enabled: Res<SoundEnabled>,
) {
    if sound_enabled.0 {
        commands.spawn(AudioBundle {
            source: audio_handles.click.clone(),
            ..default()
        });
    }
}
