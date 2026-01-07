use crate::game::anim::{AnimationIndices, AnimationTimer};

use super::{GameDynamicEntity, GameEntity, MovingObject};
use bevy::prelude::*;

//const PLATFORM_ANIMATION_SPEED: f32 = 10.0;
pub const PLATFORM_HEIGHT: f32 = 16.0;
pub const PLATFORM_WIDTH: f32 = 64.0;
pub const PLATFORM_SIZE: Vec2 = Vec2::new(PLATFORM_WIDTH, PLATFORM_HEIGHT);
const PLATFORM_VELOCITY_X: f32 = 60.0;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum PlatformState {
    #[default]
    Normal,
    Pulverizing(f32),
}

#[derive(Component, Default)]
pub struct Platform {
    pub state: PlatformState,
}

pub(super) fn spawn_platform(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
    moving: bool,
    position: Vec2,
) {
    // Load the platform's sprite sheet and create a texture atlas from it
    let platform_texture = asset_server.load("sprites/platform.png");
    let layout_handle = texture_atlases.add(TextureAtlasLayout::from_grid(
        UVec2::new(64, 16),
        1,
        4,
        None,
        None,
    ));

    let animation_indices = AnimationIndices {
        first: 0,
        last: 4,
        ..default()
    };

    // Spawn platform
    let sprite_bundle = (
        Sprite::from_atlas_image(
            platform_texture,
            TextureAtlas {
                layout: layout_handle,
                index: 0,
            },
        ),
        Transform::from_xyz(position.x, position.y, 20.0),
    );

    if moving {
        commands.spawn((
            Platform { ..default() },
            GameEntity,
            GameDynamicEntity,
            MovingObject {
                width: PLATFORM_WIDTH,
                velocity_x: PLATFORM_VELOCITY_X,
                dir: 1.0,
            },
            sprite_bundle,
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ));
    } else {
        commands.spawn((Platform { ..default() }, GameEntity, sprite_bundle));
    }
}

pub(super) fn animate_platforms(
    mut commands: Commands,
    //mut platform_query: Query<(Entity, &Platform), With<Platform>>,
    mut platform_query: Query<
        (
            &AnimationIndices,
            &mut AnimationTimer,
            Entity,
            &mut Sprite,
            &Platform,
        ),
        With<Platform>,
    >,
    time: Res<Time>,
) {
    for (indices, mut timer, entity, mut sprite, platform) in &mut platform_query {
        if let PlatformState::Pulverizing(_start_time) = platform.state {
            timer.tick(time.delta());

            if timer.just_finished()
                && let Some(atlas) = &mut sprite.texture_atlas
            {
                if atlas.index == indices.last {
                    commands.entity(entity).despawn();
                } else {
                    atlas.index += 1;
                }
            }
        }
    }
}
