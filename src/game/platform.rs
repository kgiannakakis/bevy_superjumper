use super::GameEntity;
use bevy::prelude::*;

const PLATFORM_ANIMATION_SPEED: f32 = 10.0;
pub const PLATFORM_HEIGHT: f32 = 16.0;
pub const PLATFORM_WIDTH: f32 = 64.0;
pub const PLATFORM_SIZE: Vec2 = Vec2::new(PLATFORM_WIDTH, PLATFORM_HEIGHT);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum PlatformType {
    #[default]
    Static,
    Moving,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum PlatformState {
    #[default]
    Normal,
    Pulverizing,
}

#[derive(Component, Default)]
pub struct Platform {
    platform_type: PlatformType,
    pub state: PlatformState,
}

pub(super) fn spawn_platform(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    platform_type: PlatformType,
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
    commands.spawn((
        Platform {
            platform_type,
            ..default()
        },
        GameEntity,
        SpriteSheetBundle {
            texture_atlas,
            transform: Transform::from_xyz(position.x, position.y, 20.0),
            ..Default::default()
        },
    ));
}

pub(super) fn animate_platforms(
    mut commands: Commands,
    mut platform_query: Query<(Entity, &Platform, &mut TextureAtlasSprite), With<Platform>>,
    time: Res<Time>,
) {
    for (entity, platform, mut platform_ta) in &mut platform_query {
        if platform.state == PlatformState::Pulverizing {
            let index = ((time.elapsed_seconds() * PLATFORM_ANIMATION_SPEED) as usize) % 5;
            if index >= 4 {
                commands.entity(entity).despawn();
            } else {
                platform_ta.index = index;
            }
        }
    }
}
