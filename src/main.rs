//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::camera::ScalingMode;
use bevy::{audio::Volume, prelude::*, window::WindowResolution};
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

#[derive(Message, Default)]
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
                        resolution: WindowResolution::new(400, 600),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .init_resource::<SoundEnabled>()
        .init_state::<GameState>()
        .add_message::<SoundEvent>()
        .add_systems(Startup, (scene_setup, play_music))
        .add_systems(Update, handle_sound_event)
        .add_plugins((
            bevy::diagnostic::LogDiagnosticsPlugin::default(),
            bevy::diagnostic::FrameTimeDiagnosticsPlugin::default(),
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

    // game_2d_camera.projection.scaling_mode = ScalingMode::FixedVertical(480.0);
    // game_2d_camera.transform = Transform::from_xyz(0.0, 0.0, 0.0);
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 0.0),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 480.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

    // Spawn the background sprite
    commands.spawn((
        Background,
        Sprite::from_image(asset_server.load("sprites/background.png")),
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
        commands.entity(entity).despawn();
    }
}

fn play_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sound_enabled: Res<SoundEnabled>,
) {
    if sound_enabled.0 {
        commands.spawn((
            AudioPlayer::<AudioSource>(asset_server.load("audio/music.ogg")),
            PlaybackSettings::LOOP.with_volume(Volume::Linear(0.1)),
            GameMusic,
        ));
    }
}

fn handle_sound_event(
    mut commands: Commands,
    audio_handles: Res<AudioHandles>,
    mut sound_events: MessageReader<SoundEvent>,
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
                commands.spawn(AudioPlayer::<AudioSource>(source));
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
        commands.spawn(AudioPlayer::<AudioSource>(audio_handles.click.clone()));
    }
}
