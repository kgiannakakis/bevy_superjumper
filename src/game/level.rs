use rand::Rng;

use super::{bob, coin, platform, spring, squirrel};

const WORLD_WIDTH: f32 = 10.0 * 32.0;
const WORLD_HEIGHT: f32 = 15.0 * 32.0 * 20.0;

pub enum GameObjectType {
    Platform(bool),
    Squirrel,
    Coin,
    Spring,
    Castle,
}
pub struct GameObject {
    pub object_type: GameObjectType,
    pub x: f32,
    pub y: f32,
    pub is_spawned: bool,
}

pub fn generate_level() -> Vec<GameObject> {
    let mut objects: Vec<GameObject> = Vec::new();
    let mut y: f32 = platform::PLATFORM_HEIGHT / 2.0;
    let max_jump_height: f32 =
        bob::BOB_JUMP_VELOCITY * bob::BOB_JUMP_VELOCITY / (2.0 * -bob::GRAVITY_Y);
    let mut rng = rand::thread_rng();
    let is_spawned = false;
    while y < WORLD_HEIGHT - WORLD_WIDTH / 2.0 {
        let moving = rng.gen_range(0.0..1.0) > 0.8;
        let x = rng.gen_range(0.0..1.0) * (WORLD_WIDTH - platform::PLATFORM_WIDTH)
            + platform::PLATFORM_WIDTH / 2.0;

        objects.push(GameObject {
            object_type: GameObjectType::Platform(moving),
            x,
            y,
            is_spawned,
        });

        if rng.gen_range(0.0..1.0) > 0.9 && !moving {
            objects.push(GameObject {
                object_type: GameObjectType::Spring,
                x,
                y: y + (platform::PLATFORM_HEIGHT + spring::SPRING_HEIGHT) / 2.0,
                is_spawned,
            });
        }

        if rng.gen_range(0.0..1.0) > 0.6 {
            objects.push(GameObject {
                object_type: GameObjectType::Coin,
                x: x + rng.gen_range(0.0..1.0) * 32.0,
                y: y + coin::COIN_HEIGHT + rng.gen_range(0.0..1.0) * 32.0 * 3.0,
                is_spawned,
            });
        }

        if y > WORLD_HEIGHT / 3.0 && rng.gen_range(0.0..1.0) > 0.8 {
            objects.push(GameObject {
                object_type: GameObjectType::Squirrel,
                x: x + rng.gen_range(0.0..1.0) * 32.0,
                y: y + squirrel::SQUIRREL_HEIGHT + rng.gen_range(0.0..1.0) * 32.0 * 2.0,
                is_spawned,
            });
        }

        y += max_jump_height - 0.5 * 32.0;
        y -= rng.gen_range(0.0..1.0) * (max_jump_height / 3.0);
    }

    objects.push(GameObject {
        object_type: GameObjectType::Castle,
        x: WORLD_WIDTH / 2.0,
        y,
        is_spawned,
    });

    objects
}
