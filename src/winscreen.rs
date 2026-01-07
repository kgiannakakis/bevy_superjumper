use crate::{GameState, cleanup, click_sound};
use bevy::prelude::*;

#[derive(Component)]
struct WinScreenEntity;

#[derive(Component)]
struct WinScreenTextBox;

#[derive(Component)]
struct WinScreenText;

#[derive(Resource, Default)]
pub struct WinScreenIndex(usize);

const MESSAGES: [&str; 7] = [
    "Princess: Oh dear!\n What have you done?\n\n\n\n\n\n\n",
    "Bob: I came to \nrescue you!",
    "Princess: you are\n mistaken\nI need no rescueing",
    "Bob: So all this \nwork for nothing?",
    "Princess: I have \ncake and tea!\nWould you like some?",
    "Bob: I'd be my \npleasure!",
    "And they ate cake\nand drank tea\nhappily ever \nafter\n\n\n\n\n\n\n\nKära Emma!\nDu är fantastisk!\nDu blev ferdig\n med spelet!",
];

pub struct WinScreenPlugin;
impl Plugin for WinScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WinScreenIndex>()
            .add_systems(OnEnter(GameState::WinScreen), setup_winscreen)
            .add_systems(OnExit(GameState::WinScreen), cleanup::<WinScreenEntity>)
            .add_systems(
                Update,
                (
                    show_next_screen.run_if(in_state(GameState::WinScreen).and(has_user_input)),
                    click_sound.run_if(
                        resource_changed::<WinScreenIndex>
                            .and(not(resource_added::<WinScreenIndex>)),
                    ),
                ),
            );
    }
}

fn setup_winscreen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((
        WinScreenEntity,
        Sprite::from_image(asset_server.load("sprites/castle.png")),
        Transform::from_xyz(0.0, 0.0, 100.0).with_scale(Vec3::new(3.0, 3.0, 1.0)),
    ));

    // Load the bob's sprite sheet and create a texture atlas from it
    let bob_texture = asset_server.load("sprites/bob.png");
    let layout_handle = texture_atlases.add(TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),
        5,
        1,
        None,
        None,
    ));

    // Spawn bob
    commands.spawn((
        WinScreenEntity,
        Sprite::from_atlas_image(
            bob_texture,
            TextureAtlas {
                layout: layout_handle,
                index: 0,
            },
        ),
        Transform::from_xyz(-20.0, -5.0, 110.0),
    ));

    commands.spawn((
        WinScreenEntity,
        Sprite::from_image(asset_server.load("sprites/princess.png")),
        Transform::from_xyz(20.0, -5.0, 110.0)
            .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
    ));

    commands
        .spawn((
            WinScreenTextBox,
            WinScreenEntity,
            Node {
                width: Val::Vw(100.0),
                height: Val::Vh(100.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|builder| {
            builder.spawn((
                Text::new(MESSAGES[0]),
                TextFont {
                    font: asset_server.load("fonts/Retroville NC.ttf"),
                    font_size: 26.0,
                    ..default()
                },
                TextLayout::new_with_justify(Justify::Center),
                TextColor(Color::WHITE),
                Transform::from_translation(Vec3::Z),
                WinScreenText,
            ));
        });
}

fn show_next_screen(
    query: Query<Entity, With<WinScreenText>>,
    mut writer: TextUiWriter,
    mut win_screen: ResMut<WinScreenIndex>,
    mut state: ResMut<NextState<GameState>>,
) {
    win_screen.0 += 1;
    if win_screen.0 < 7 {
        let entity = query.single().unwrap();
        *writer.text(entity, 0) = MESSAGES[win_screen.0].to_string();
    } else {
        win_screen.0 = 0;
        state.set(GameState::Menu);
    }
}

pub fn has_user_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    touch_input: Res<Touches>,
) -> bool {
    keyboard_input.just_pressed(KeyCode::Space)
        || mouse_button_input.just_pressed(MouseButton::Left)
        || touch_input.any_just_pressed()
}
