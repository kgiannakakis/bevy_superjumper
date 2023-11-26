#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{audio::Volume, prelude::*, window::WindowResolution};

mod help;
mod menu;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Playing,
    //GameOver,
    Help,
    HighScores,
}

#[derive(Component)]
struct GameMusic;

#[derive(Resource)]
struct AudioHandles {
    click: Handle<AudioSource>,
    // coin: Handle<AudioSource>,
    // jump: Handle<AudioSource>,
    // highjump: Handle<AudioSource>,
    // hit: Handle<AudioSource>,
}

#[derive(Resource, Default)]
pub struct SoundDisabled(bool);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Super Jumper"),
                        resolution: WindowResolution::new(320.0, 480.0),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_state::<GameState>()
        .add_systems(Startup, (scene_setup, play_music))
        // .add_plugin(game::GamePlugin)
        // .add_plugin(game_over::GameOverPlugin)
        .add_plugins((menu::MenuPlugin, help::HelpPlugin))
        .run();
}

fn scene_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn a 2D camera
    commands.spawn(Camera2dBundle::default());

    // Spawn the background sprite
    commands.spawn(SpriteBundle {
        texture: asset_server.load("sprites/background.png"),
        ..Default::default()
    });

    // Load audio files
    commands.insert_resource(AudioHandles {
        // coin: asset_server.load("audio/coin.ogg"),
        // hit: asset_server.load("audio/hit.ogg"),
        // jump: asset_server.load("audio/jump.ogg"),
        // highjump: asset_server.load("audio/highjump.ogg"),
        click: asset_server.load("audio/click.ogg"),
    });
}

// Despawn all entities recursively with a given component
pub fn cleanup<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn play_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("audio/music.ogg"),
            settings: PlaybackSettings::LOOP.with_volume(Volume::new_relative(0.1)),
        },
        GameMusic,
    ));
}

fn click_sound(
    audio_handles: Res<AudioHandles>,
    mut commands: Commands,
    sound_disabled: Res<SoundDisabled>,
) {
    if !sound_disabled.0 {
        commands.spawn(AudioBundle {
            source: audio_handles.click.clone(),
            ..default()
        });
    }
}
