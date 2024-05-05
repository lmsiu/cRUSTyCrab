use bevy::{prelude::*, math::vec3};

// Constant Variables

// Paddle Variables
const PADDLE_START_Y: f32 = 0.0;
const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const PADDLE_SPEED: f32 = 500.0;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup) //these systems are really just functions
        .add_systems(FixedUpdate, move_paddle)// runs at a fixed rate
        .run()
}

// The class for the paddle object
#[derive(Component)]
struct Paddle;

fn setup(mut commands: Commands){
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
}

fn move_paddle(
    input: Res<ButtonInput<KeyCode>>,
    time_step: Res<Time>,
    mut query: Query<&mut Transform, With<Paddle>>,
){
    let mut paddle_transform = query.single_mut();
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



