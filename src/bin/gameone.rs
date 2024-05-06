use std::char::from_u32;
use bevy::{prelude::*, math::vec3, };
use  bevy::math::bounding::{IntersectsVolume};

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

// scoreboard
const SCOREBOARD_FONT_SIZE:f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Scoreboard{score: 0})
        .add_systems(Update, (bevy::window::close_on_esc, update_scoreboard))
        .add_systems(Startup, setup) //these systems are really just functions
        .add_systems(FixedUpdate,
                     (move_paddle,
                      apply_velocity,
                      check_all_crab_collisions.after(apply_velocity),
                      check_crab_collisions.after(apply_velocity),))// runs at a fixed rate
        .run()
}

// The class for the paddle object
#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Crab{
    size: Vec2,
}

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

#[derive(Resource, Clone, Copy)]
struct Scoreboard{
    score: usize,
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
         Crab{ size: CRAB_SIZE },
        Velocity(CRAB_SPEED * CRAB_INITIAL_DIRECTION)
        ) // Add paddle component to the player
    );

    // spawn box from walls
    {
        let vertical_wall_size: Vec2 = Vec2::new(WALL_THICKNESS, WALL_BLOCK_HEIGHT + WALL_THICKNESS);
        let horizantal_wall_size: Vec2 = Vec2::new(WALL_BLOCK_WIDTH + WALL_THICKNESS, WALL_THICKNESS);

        // left wall
        commands.spawn(WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: vec3(LEFT_WALL, 0.0, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    custom_size: Some(vertical_wall_size),
                    ..default()
                },
                ..default()
            },
            collider: Collider {
                size: vertical_wall_size,
            }
        });

        // right wall
        commands.spawn(WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: vec3(RIGHT_WALL, 0.0, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    custom_size: Some(vertical_wall_size),
                    ..default()
                },
                ..default()
            },
            collider: Collider {
                size: vertical_wall_size,
            }
        });

        // bottom wall
        commands.spawn(WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: vec3(0.0, BOTTOM_WALL, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    custom_size: Some(horizantal_wall_size),
                    ..default()
                },
                ..default()
            },
            collider: Collider {
                size: horizantal_wall_size,
            }
        });

        //top wall
        commands.spawn(WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: vec3(0.0, TOP_WALL, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    custom_size: Some(horizantal_wall_size),
                    ..default()
                },
                ..default()
            },
            collider: Collider {
                size: horizantal_wall_size,
            }
        });
    }

        // scoreboard
        commands.spawn((TextBundle::from_sections([
            TextSection::new(
            "Score: ",
            TextStyle{
                font_size: SCOREBOARD_FONT_SIZE,
                color: TEXT_COLOR,
                ..default()
            }),
            TextSection::from_style(TextStyle{
                font_size: SCOREBOARD_FONT_SIZE,
                color: TEXT_COLOR,
                ..default()
            }),
        ]).with_style(
            Style{
                position_type: PositionType::Absolute,
                top: SCOREBOARD_TEXT_PADDING,
                left: SCOREBOARD_TEXT_PADDING,
                ..default()

            }
        ),));

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

    let mut new_x: f32 = paddle_transform.translation.x + direction * PADDLE_SPEED * time_step.delta_seconds();

    // "Collision"
    new_x = new_x.min(RIGHT_WALL - (WALL_THICKNESS+PADDLE_SIZE.x) * 0.5); // take either the new x position or the wall
    new_x = new_x.max(LEFT_WALL + (WALL_THICKNESS+PADDLE_SIZE.x) * 0.5); // take either the new x position or the wall position

    paddle_transform.translation.x = new_x;
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<Time>){
    let dt = time_step.delta_seconds();
    for(mut transform, velocity) in &mut query{
        transform.translation.x += velocity.x * dt;
        transform.translation.y += velocity.y * dt;
    }

}

fn check_crab_collisions(
    mut crab_query: Query<(&mut Velocity, &Transform, &Crab)>
){
    let x_min: f32 = LEFT_WALL + (WALL_THICKNESS+CRAB_SIZE.x) * 0.5;
    let x_max: f32 = RIGHT_WALL - (WALL_THICKNESS+CRAB_SIZE.x) * 0.5;
    let y_min: f32 = BOTTOM_WALL + (WALL_THICKNESS+CRAB_SIZE.y) * 0.5;
    let y_max: f32 = TOP_WALL - (WALL_THICKNESS+CRAB_SIZE.y) * 0.5;

    for(mut crab_velocity, crab_transform, crab) in &mut crab_query {
        let translation: Vec3 = crab_transform.translation;

        if (translation.x < x_min || translation.x > x_max) {
            crab_velocity.x *= -1.
        }

        if (translation.y < y_min || translation.y > y_max) {
            crab_velocity.y *= -1.;
        }
    }
}

fn check_all_crab_collisions(
    mut crab_query: Query<(&mut Velocity, &Transform, &Crab)>,
    mut score: ResMut<Scoreboard>,
    paddle_query: Query<(&Transform, &Paddle)>,
){
    for(mut crab_velocity, crab_transform, crab) in &mut crab_query {
        for(transform, other) in &paddle_query{
            let x_min: f32 = transform.translation.x - (PADDLE_SIZE.x/2.0);
            let x_max: f32 = transform.translation.x + (PADDLE_SIZE.x/2.0);
            let y_min: f32 = transform.translation.y - (PADDLE_SIZE.y+ CRAB_SIZE.y) * 0.5;
            let y_max: f32 = transform.translation.y + (PADDLE_SIZE.y+CRAB_SIZE.y) * 0.5;

            let translation: Vec3 = crab_transform.translation;

            if (translation.x >= x_min && translation.x <= x_max) {
                if (translation.y >= y_min && translation.y <= y_max) {
                    crab_velocity.y *= -1.;
                    score.score += 1;
                }
            }

        }

    }
}

fn update_scoreboard(score: Res<Scoreboard>, mut query: Query<&mut Text>){
    let mut text = query.single_mut();
    text.sections[1].value = score.score.to_string();

}





