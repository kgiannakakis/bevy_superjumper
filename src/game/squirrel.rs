use crate::game::anim::{AnimationIndices, AnimationTimer};

use super::{GameDynamicEntity, GameEntity, MovingObject};
use bevy::prelude::*;

//const SQUIRREL_ANIMATION_SPEED: f32 = 10.0;
pub const SQUIRREL_HEIGHT: f32 = 0.6 * 32.0;
pub const SQUIRREL_WIDTH: f32 = 32.0;
pub const SQUIRREL_SIZE: Vec2 = Vec2::new(SQUIRREL_WIDTH, SQUIRREL_HEIGHT);
const SQUIRREL_VELOCITY_X: f32 = 60.0;

#[derive(Component, Default)]
pub struct Squirrel;

pub(super) fn spawn_squirrel(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
    position: Vec2,
) {
    // Load the squirrel's sprite sheet and create a texture atlas from it
    let squirrel_texture = asset_server.load("sprites/squirrel.png");
    let layout_handle = texture_atlases.add(TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),
        2,
        1,
        None,
        None,
    ));

    let animation_indices = AnimationIndices {
        first: 0,
        last: 1,
        ..default()
    };

    // Spawn coin
    commands.spawn((
        Squirrel,
        GameEntity,
        GameDynamicEntity,
        MovingObject {
            width: SQUIRREL_WIDTH,
            velocity_x: SQUIRREL_VELOCITY_X,
            dir: 1.0,
        },
        Sprite::from_atlas_image(
            squirrel_texture,
            TextureAtlas {
                layout: layout_handle,
                index: 0,
            },
        ),
        Transform::from_xyz(position.x, position.y, 20.0),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

pub(super) fn animate_squirrels(
    mut squirrels: Query<
        (
            &AnimationIndices,
            &mut AnimationTimer,
            &mut Sprite,
            &mut Transform,
            &MovingObject,
        ),
        With<Squirrel>,
    >,
    time: Res<Time>,
) {
    for (indices, mut timer, mut sprite, mut transform, squirrel) in &mut squirrels {
        timer.tick(time.delta());

        if timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }

        if squirrel.dir < 0.0 {
            transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
        } else {
            transform.rotation = Quat::default();
        }
    }
}
