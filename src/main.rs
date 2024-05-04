use std::ptr::null;

use bevy::prelude::*;

const AJD: i32 = 5;

static mut WALK1: Handle<Image>;
static mut WALK2: Handle<Image>;

fn update(
    mut characters: Query<(&mut Transform, &Sprite)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, mut spr) in &mut characters {
        let oldX = transform.translation.x;
        let oldY = transform.translation.y;
        if input.pressed(KeyCode::KeyW) {
            transform.translation.y += 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::KeyS) {
            transform.translation.y -= 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::KeyD) {
            transform.translation.x += 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::KeyA) {
            transform.translation.x -= 100.0 * time.delta_seconds();
        }

        if oldX != transform.translation.x || oldY != transform.translation.y {
            spr.texture = WALK1;
        }

    }
}

#[derive(Component)]
struct Person;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //commands.spawn(Camera2dBundle::default());
    commands.spawn(Camera2dBundle::default());

    let texture = asset_server.load("elephant.png");
    WALK1 = asset_server.load("walk1.png");
    WALK2 = asset_server.load("walk2.png");



    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        texture: texture,
        ..default()
    });

    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

#[derive(Component)]
struct Name(String);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Logic Farming Rougelike".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();

    /*App::new().add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.3, 0.6)))
        .add_systems(Startup, setup)
        .add_systems(Update, hello_world)
        .run();*/
}

    