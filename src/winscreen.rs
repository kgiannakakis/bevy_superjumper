use crate::{cleanup, click_sound, GameState};
use bevy::{
    prelude::*,
    text::{BreakLineOn, Text2dBounds},
};

#[derive(Component)]
struct WinScreenEntity;

#[derive(Component)]
struct WinScreenTextBox;

#[derive(Resource, Default)]
pub struct WinScreenIndex(usize);

const MESSAGES: [&str; 7] = [ "Princess: Oh dear!\n What have you done?\n\n\n\n\n\n\n",
"Bob: I came to \nrescue you!",
"Princess: you are\n mistaken\nI need no rescueing",
"Bob: So all this \nwork for nothing?",
"Princess: I have \ncake and tea!\nWould you like some?",
"Bob: I'd be my \npleasure!",
"And they ate cake\nand drank tea\nhappily ever \nafter\n\n\n\n\n\n\n\nKära Emma!\nDu är fantastisk!\nDu blev ferdig\n med spelet!"];

const TRANSPARENT: Color = Color::Rgba {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
    alpha: 0.0,
};

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

fn setup_winscreen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/castle.png"),
            transform: Transform::from_xyz(0.0, 0.0, 100.0).with_scale(Vec3::new(3.0, 3.0, 1.0)),
            ..Default::default()
        },
        WinScreenEntity,
    ));

    // Load the bob's sprite sheet and create a texture atlas from it
    let bob_texture = asset_server.load("sprites/bob.png");
    let texture_atlas = texture_atlases.add(TextureAtlas::from_grid(
        bob_texture,
        Vec2::new(32.0, 32.0),
        5,
        1,
        None,
        None,
    ));

    // Spawn bob
    commands.spawn((
        WinScreenEntity,
        SpriteSheetBundle {
            texture_atlas,
            transform: Transform::from_xyz(-20.0, -5.0, 110.0),
            ..Default::default()
        },
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/princess.png"),
            transform: Transform::from_xyz(20.0, -5.0, 110.0)
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            ..Default::default()
        },
        WinScreenEntity,
    ));

    let box_size = Vec2::new(320.0, 400.0);
    let box_position = Vec2::new(0.0, 160.0);
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: TRANSPARENT,
                    custom_size: Some(Vec2::new(box_size.x, box_size.y)),
                    ..default()
                },
                transform: Transform::from_translation(box_position.extend(0.0)),
                ..default()
            },
            WinScreenTextBox,
            WinScreenEntity,
        ))
        .with_children(|builder| {
            builder.spawn(Text2dBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        MESSAGES[0],
                        TextStyle {
                            font: asset_server.load("fonts/Retroville NC.ttf"),
                            font_size: 26.0,
                            color: Color::WHITE,
                        },
                    )],
                    alignment: TextAlignment::Center,
                    linebreak_behavior: BreakLineOn::WordBoundary,
                },
                text_2d_bounds: Text2dBounds {
                    // Wrap text in the rectangle
                    size: box_size,
                },
                // ensure the text is drawn on top of the box
                transform: Transform::from_translation(Vec3::Z),
                ..default()
            });
        });
}

fn show_next_screen(
    asset_server: Res<AssetServer>,
    mut text_query: Query<(Entity, &mut Text)>,
    mut text_box_query: Query<&mut Transform, With<WinScreenTextBox>>,
    mut win_screen: ResMut<WinScreenIndex>,
    mut state: ResMut<NextState<GameState>>,
) {
    win_screen.0 += 1;
    if win_screen.0 < 7 {
        let mut text = text_query.single_mut();
        text.1.sections = vec![TextSection::new(
            MESSAGES[win_screen.0],
            TextStyle {
                font: asset_server.load("fonts/Retroville NC.ttf"),
                font_size: 26.0,
                color: Color::WHITE,
            },
        )];
        if win_screen.0 == 6 {
            let mut text_box = text_box_query.single_mut();
            *text_box = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
        }
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
