use crate::{cleanup, click_sound, GameMusic, GameState, SoundDisabled};
use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component)]
struct MenuEntity;

#[derive(Component)]
struct SoundButtonEntity;

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SoundDisabled>()
            .add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(
                OnExit(GameState::Menu),
                (click_sound, cleanup::<MenuEntity>),
            )
            .add_systems(
                Update,
                (
                    start_playing.run_if(in_state(GameState::Menu).and_then(has_clicked_play)),
                    show_help.run_if(in_state(GameState::Menu).and_then(has_clicked_help)),
                    show_highscores
                        .run_if(in_state(GameState::Menu).and_then(has_clicked_highscores)),
                    toggle_pause_music
                        .run_if(in_state(GameState::Menu).and_then(has_clicked_sound)),
                ),
            );
    }
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/logo.png"),
            transform: Transform::from_xyz(0.0, 240.0 - 10.0 - 142.0 / 2.0, 100.0),
            ..Default::default()
        },
        MenuEntity,
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/mainMenu.png"),
            transform: Transform::from_xyz(0.0, -40.0, 100.0),
            ..Default::default()
        },
        MenuEntity,
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/soundOn.png"),
            transform: Transform::from_xyz(-160.0 + 10.0 + 32.0, -240.0 + 10.0 + 32.0, 100.0),
            ..Default::default()
        },
        (MenuEntity, SoundButtonEntity),
    ));
}

fn start_playing(mut state: ResMut<NextState<GameState>>) {
    println!("Play");
    state.set(GameState::Playing);
}

fn show_help(mut state: ResMut<NextState<GameState>>) {
    println!("Help");
    state.set(GameState::Help);
}

fn show_highscores(mut state: ResMut<NextState<GameState>>) {
    println!("High Scores");
    state.set(GameState::HighScores);
}

pub fn has_clicked_play(
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button_input: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) -> bool {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Some(position) = q_windows.single().cursor_position() {
            return play_button_pressed(position);
        }
    }

    keyboard_input.just_pressed(KeyCode::P)
}

pub fn has_clicked_highscores(
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button_input: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) -> bool {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Some(position) = q_windows.single().cursor_position() {
            return highscores_button_pressed(position);
        }
    }

    keyboard_input.just_pressed(KeyCode::S)
}

pub fn has_clicked_help(
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button_input: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) -> bool {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Some(position) = q_windows.single().cursor_position() {
            return help_button_pressed(position);
        }
    }

    keyboard_input.just_pressed(KeyCode::H)
}

pub fn has_clicked_sound(
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button_input: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) -> bool {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Some(position) = q_windows.single().cursor_position() {
            return sound_button_pressed(position);
        }
    }

    keyboard_input.just_pressed(KeyCode::A)
}

fn play_button_pressed(position: Vec2) -> bool {
    position.y > 240.0 + 40.0 - 55.0 && position.y < 240.0 + 40.0 - 55.0 + 35.0
}

fn highscores_button_pressed(position: Vec2) -> bool {
    position.y > 240.0 + 40.0 - 55.0 + 35.0 && position.y < 240.0 + 40.0 - 55.0 + 70.0
}

fn help_button_pressed(position: Vec2) -> bool {
    position.y > 240.0 + 40.0 - 55.0 + 70.0 && position.y < 240.0 + 40.0 - 55.0 + 110.0
}

fn sound_button_pressed(position: Vec2) -> bool {
    position.x > 10.0
        && position.x < 10.0 + 64.0
        && position.y > 480.0 - 10.0 - 64.0
        && position.y < 480.0 - 10.0
}

fn toggle_pause_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut sound_disabled: ResMut<SoundDisabled>,
    sound_button_query: Query<Entity, With<SoundButtonEntity>>,
    music_query: Query<&AudioSink, With<GameMusic>>,
) {
    if let Ok(sink) = music_query.get_single() {
        sink.toggle();
    }

    sound_disabled.0 = !sound_disabled.0;

    let path = match sound_disabled.0 {
        true => "sprites/soundOff.png",
        false => "sprites/soundOn.png",
    };

    for sound_button_entity in &sound_button_query {
        commands.entity(sound_button_entity).despawn();
    }

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(path),
            transform: Transform::from_xyz(-160.0 + 10.0 + 32.0, -240.0 + 10.0 + 32.0, 100.0),
            ..Default::default()
        },
        (MenuEntity, SoundButtonEntity),
    ));
}
