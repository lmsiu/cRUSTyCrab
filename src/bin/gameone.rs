use bevy::{prelude::*, math::vec3};

// Constant Variables

// Paddle Variables
const PADDLE_START_Y: f32 = 0.0;
const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const PADDLE_SPEED: f32 = 500.0;

// crab
const CRAB_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const CRAB_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const CRAB_SPEED: f32 = 400.0;
const CRAB_INITIAL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

// Box for the game
const LEFT_WALL: f32 = -450.0;
const RIGHT_WALL: f32 = 450.0;
const BOTTOM_WALL: f32 = -300.0;
const TOP_WALL: f32 = 300.0;
const WALL_THICKNESS: f32 = 10.0;
const WALL_BLOCK_WIDTH: f32 = RIGHT_WALL - LEFT_WALL;
const WALL_BLOCK_HEIGHT: f32 = TOP_WALL - BOTTOM_WALL;
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup) //these systems are really just functions
        .add_systems(FixedUpdate, (move_paddle, apply_velocity))// runs at a fixed rate
        .run()
}

// The class for the paddle object
#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Crab;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider{
    size: Vec2,
}

#[derive(Bundle)]
struct WallBundle{
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>){
    // make the game camera
    commands.spawn(Camera2dBundle::default());

    // spawn the paddle
    commands.spawn(
        (SpriteBundle{
            transform: Transform{
                translation: vec3(0., PADDLE_START_Y, 0.),
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                custom_size: Some(PADDLE_SIZE),
                ..default()
            },
            ..default()
        },
        Paddle,) // Add paddle component to the player
        );

    // spawn the crab
    let crab_texture = asset_server.load("textures\\rustacean-flat-happy.png");
    commands.spawn(
        (SpriteBundle{
            transform: Transform{
                translation: CRAB_STARTING_POSITION,
                ..default()
            },
            sprite: Sprite {
                //color: CRAB_COLOR,
                custom_size: Some(CRAB_SIZE),
                ..default()
            },
            texture: crab_texture,
            ..default()
        },
         Crab,
        Velocity(CRAB_SPEED * CRAB_INITIAL_DIRECTION)
        ) // Add paddle component to the player
    );

    // spawn box from walls
    {
        let vertical_wall_size: Vec2 = Vec2::new(WALL_THICKNESS, WALL_BLOCK_HEIGHT + WALL_THICKNESS);
        let horizantal_wall_size: Vec2 = Vec2::new(WALL_BLOCK_WIDTH + WALL_THICKNESS, WALL_THICKNESS);

        // left wall
        commands.spawn(WallBundle{
            sprite_bundle: SpriteBundle{
                transform: Transform{
                    translation: vec3(LEFT_WALL, 0.0, 0.0),
                    ..default()
                },
                sprite: Sprite{
                    color: WALL_COLOR,
                    custom_size: Some(vertical_wall_size),
                    ..default()
                },
                ..default()
            },
            collider: Collider{
                size: vertical_wall_size,
            }


        });

        // right wall
        commands.spawn(WallBundle{
            sprite_bundle: SpriteBundle{
                transform: Transform{
                    translation: vec3(RIGHT_WALL, 0.0, 0.0),
                    ..default()
                },
                sprite: Sprite{
                    color: WALL_COLOR,
                    custom_size: Some(vertical_wall_size),
                    ..default()
                },
                ..default()
            },
            collider: Collider{
                size: vertical_wall_size,
            }
        });

        // bottom wall
        commands.spawn(WallBundle{
            sprite_bundle: SpriteBundle{
                transform: Transform{
                    translation: vec3(0.0, BOTTOM_WALL, 0.0),
                    ..default()
                },
                sprite: Sprite{
                    color: WALL_COLOR,
                    custom_size: Some(horizantal_wall_size),
                    ..default()
                },
                ..default()
            },
            collider: Collider{
                size: horizantal_wall_size,
            }
        });

        //top wall
        commands.spawn(WallBundle{
            sprite_bundle: SpriteBundle{
                transform: Transform{
                    translation: vec3(0.0, TOP_WALL, 0.0),
                    ..default()
                },
                sprite: Sprite{
                    color: WALL_COLOR,
                    custom_size: Some(horizantal_wall_size),
                    ..default()
                },
                ..default()
            },
            collider: Collider{
                size: horizantal_wall_size,
            }
        });

    }
}

fn move_paddle(
    input: Res<ButtonInput<KeyCode>>,
    time_step: Res<Time>,
    mut query: Query<&mut Transform, With<Paddle>>, // gives reference to the paddle
){
    let mut paddle_transform = query.single_mut(); // makes the paddle a singleton
    let mut direction = 0.0;

    if input.pressed(KeyCode::ArrowLeft){
        direction -= 1.0;
    }
    if input.pressed(KeyCode::ArrowRight){
        direction += 1.0;
    }

    let new_x = paddle_transform.translation.x + direction * PADDLE_SPEED * time_step.delta_seconds();

    paddle_transform.translation.x = new_x;
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<Time>){
    let dt = time_step.delta_seconds();
    for(mut transform, velocity) in &mut query{
        transform.translation.x += velocity.x * dt;
        transform.translation.y += velocity.y * dt;
    }

}




