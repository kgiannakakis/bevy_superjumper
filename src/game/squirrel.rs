use super::{GameEntity, MovingObject};
use bevy::prelude::*;

const SQUIRREL_ANIMATION_SPEED: f32 = 10.0;
pub const SQUIRREL_HEIGHT: f32 = 0.6 * 32.0;
pub const SQUIRREL_WIDTH: f32 = 32.0;
pub const SQUIRREL_SIZE: Vec2 = Vec2::new(SQUIRREL_WIDTH, SQUIRREL_HEIGHT);
const SQUIRREL_VELOCITY_X: f32 = 60.0;

#[derive(Component, Default)]
pub struct Squirrel;

pub(super) fn spawn_squirrel(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    position: Vec2,
) {
    // Load the squirrel's sprite sheet and create a texture atlas from it
    let squirrel_texture = asset_server.load("sprites/squirrel.png");
    let texture_atlas = texture_atlases.add(TextureAtlas::from_grid(
        squirrel_texture,
        Vec2::new(32.0, 32.0),
        2,
        1,
        None,
        None,
    ));

    // Spawn coin
    commands.spawn((
        Squirrel,
        GameEntity,
        MovingObject {
            width: SQUIRREL_WIDTH,
            velocity_x: SQUIRREL_VELOCITY_X,
            dir: 1.0,
        },
        SpriteSheetBundle {
            texture_atlas,
            transform: Transform::from_xyz(position.x, position.y, 20.0),
            ..Default::default()
        },
    ));
}

pub(super) fn animate_squirrels(
    mut squirrels: Query<(&mut TextureAtlasSprite, &mut Transform, &MovingObject), With<Squirrel>>,
    time: Res<Time>,
) {
    for (mut squirrel_ta, mut transform, squirrel) in &mut squirrels {
        squirrel_ta.index = (time.elapsed_seconds() * SQUIRREL_ANIMATION_SPEED) as usize % 2;

        if squirrel.dir < 0.0 {
            transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
        } else {
            transform.rotation = Quat::default();
        }
    }
}
