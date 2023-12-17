use super::{GameDynamicEntity, GameEntity, MovingObject};
use bevy::prelude::*;

const PLATFORM_ANIMATION_SPEED: f32 = 10.0;
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
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    moving: bool,
    position: Vec2,
) {
    // Load the platform's sprite sheet and create a texture atlas from it
    let platform_texture = asset_server.load("sprites/platform.png");
    let texture_atlas = texture_atlases.add(TextureAtlas::from_grid(
        platform_texture,
        Vec2::new(64.0, 16.0),
        1,
        4,
        None,
        None,
    ));

    // Spawn platform
    let sprite_bundle = SpriteSheetBundle {
        texture_atlas,
        transform: Transform::from_xyz(position.x, position.y, 20.0),
        ..Default::default()
    };

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
        ));
    } else {
        commands.spawn((Platform { ..default() }, GameEntity, sprite_bundle));
    }
}

pub(super) fn animate_platforms(
    mut commands: Commands,
    mut platform_query: Query<(Entity, &Platform, &mut TextureAtlasSprite), With<Platform>>,
    time: Res<Time>,
) {
    for (entity, platform, mut platform_ta) in &mut platform_query {
        if let PlatformState::Pulverizing(start_time) = platform.state {
            let index = 1
                + (((time.elapsed_seconds() - start_time) * PLATFORM_ANIMATION_SPEED) as usize) % 5;
            if index >= 4 {
                commands.entity(entity).despawn();
            } else {
                platform_ta.index = index;
            }
        }
    }
}
