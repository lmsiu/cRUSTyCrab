use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy::render::render_resource::Texture;
use bevy::window::WindowResized;
use rand::Rng;

use std::time::{Duration, SystemTime, UNIX_EPOCH};


static mut WIDTH: f32 = 1280.0;
static mut HEIGHT: f32 = 720.0;

//const PROJECTILE_WIDTH: i32 = 100;
//const PROJECTILE_HEIGHT: i32 = 100;
//const PLAYER_WIDTH: i32 = 100;
//const PLAYER_HEIGHT: i32 = 100;

const PROJECTILE_SCALE: f32 = 0.5;

const PLAYER_SIZE: Vec2 = Vec2::new(0.75*460.0, 0.75*246.0);

#[derive(Resource)]
pub struct TextureAssets {
    pub textures: Vec<Handle<Image>>,
    pub sizes: Vec<Vec2>,
}

// Function to load textures
fn load_textures(asset_server: AssetServer) -> TextureAssets {
    let mut textures = Vec::new();

    textures.push(asset_server.load("food1.png"));
    textures.push(asset_server.load("food2.png"));
    textures.push(asset_server.load("food3.png"));
    textures.push(asset_server.load("food4.png"));
    textures.push(asset_server.load("food5.png"));
    textures.push(asset_server.load("harmful1.png"));
    textures.push(asset_server.load("harmful2.png"));
    textures.push(asset_server.load("harmful3.png"));

    let s = PROJECTILE_SCALE;
    let mut sizes = vec![Vec2::new(300.0 * s, 185.0 * s),   // food1
                                    Vec2::new(300.0 * s, 113.0 * s),   // food2
                                    Vec2::new(300.0 * s, 261.0 * s),   // food3
                                    Vec2::new(300.0 * s, 153.0 * s),   // food4
                                    Vec2::new(300.0 * s, 237.0 * s),   // food5
                                    Vec2::new(300.0 * s, 179.0 * s),   // harmful1
                                    Vec2::new(300.0 * s, 300.0 * s),   // harmful2
                                    Vec2::new(255.0 * s, 300.0 * s)];  // harmful3

    TextureAssets { textures, sizes }
}

fn seconds_since_epoch() -> u64 {
    let now = SystemTime::now();
    let duration_since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    duration_since_epoch.as_secs()
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_systems(Startup, setup)
        .insert_resource(TextureAssets { textures: Vec::new(), sizes: Vec::new() })
        .add_systems(
            Update,
            (
                spawn_sprite.run_if(on_timer(Duration::from_secs(1))),
            )
        )
        .add_systems(Update, update_player)
        .add_systems(Update, resize_notificator)
        //.add_systems(Update, update_projectiles)
        .run();
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn resize_notificator(resize_event: Res<Events<WindowResized>>) {
    let mut reader = resize_event.get_reader();
    for e in resize_event.iter_current_update_events() {
        println!("width = {} height = {}", e.width, e.height);
        unsafe {
            WIDTH = e.width;
            HEIGHT = e.height;
        }
    }
}
struct ObjectBounds {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

fn calculate_bounds(transform: &Transform, sprite: &Sprite, texture: &Texture) -> ObjectBounds {
    let scale = transform.scale.x; // Assuming uniform scaling
    let texture_size = Vec2::new(texture.width() as f32, texture.height() as f32);
    let sprite_size = texture_size * scale;
    let translation = transform.translation.truncate();

    let left = translation.x - sprite_size.x / 2.0;
    let right = translation.x + sprite_size.x / 2.0;
    let top = translation.y + sprite_size.y / 2.0;
    let bottom = translation.y - sprite_size.y / 2.0;

    ObjectBounds {
        left,
        right,
        top,
        bottom,
    }
}

/*fn hit_detection(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform, Enemy)>,
    bullet_query: Query<&Transform, With<Bullet>>
) {
    for (entity, enemy_transform) in enemy_query.iter() {
        for bullet_transform in bullet_query.iter() {
            // Your collision check
            if ... {
                commands.entity(entity).despawn();
            }
        }
    }
}*/

/*
fn update_projectiles(time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (
            &mut Transform,
            &Sprite,
            &Projectile,
        ),
    >,
) {
    for (mut transform, sprite, projectile) in &mut query {
        if input.pressed(KeyCode::KeyW) {
            transform.translation.y -= 350.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::KeyS) {
            transform.translation.y += 350.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::KeyD) {
            transform.translation.x -= 350.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::KeyA) {
            transform.translation.x += 350.0 * time.delta_seconds();
        }
    }
}*/
fn spawn_sprite(mut commands: Commands, texture_assets: Res<TextureAssets>,) {
    let index = rand::thread_rng().gen_range(0..texture_assets.textures.len());

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(texture_assets.sizes[index]),
                ..default()
            },
            //transform: Transform::from_scale(Vec3::splat(0.5)),
            texture: texture_assets.textures[index].clone_weak(),
            ..default()
        },
        Projectile {
            good: true,
            size: texture_assets.sizes[index],
        },
    ));
}


fn update_player(
    mut commands: Commands,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (
            Option<&Player>,
            Option<&Projectile>,

            Option<&AnimationIndices>, 
            Option<&mut AnimationTimer>, 
            Option<&mut TextureAtlas>,
            
            &mut Transform,
            &Sprite,
        ),
    >,
) {
    let mut player_transform;
    for (player, projectile, indices, mut timer, mut atlas, mut transform, sprite,) in &mut query
    {
        if let (Some(player), Some(indices), Some(mut timer), Some(mut atlas)) = (player, indices, timer.as_mut(), atlas.as_mut())   // if this is a PLAYER
        {
            player_transform = transform.clone();
// ======= FERRIS WALKING CODE ========
            let old_x = transform.translation.x;
            let old_y = transform.translation.y;
            if input.pressed(KeyCode::KeyW) {
                transform.translation.y += 350.0 * time.delta_seconds();
            }
            if input.pressed(KeyCode::KeyS) {
                transform.translation.y -= 350.0 * time.delta_seconds();
            }
            if input.pressed(KeyCode::KeyD) {
                transform.translation.x += 350.0 * time.delta_seconds();
            }
            if input.pressed(KeyCode::KeyA) {
                transform.translation.x -= 350.0 * time.delta_seconds();
            }
            if old_x != transform.translation.x || old_y != transform.translation.y {
                timer.tick(time.delta());
                if timer.just_finished() {
                    atlas.index = if atlas.index == indices.last {
                        indices.first + 1
                    } else {
                        atlas.index + 1
                    };
                }
                println!("X = {} Y = {}", transform.translation.x, transform.translation.y);
            } else {
                atlas.index = indices.first;
            }
        }
    }
// ======= PROJECTILE MOVEMENT AND COLLISION DETECTION CODE ========
    for (player, projectile, indices, mut timer, mut atlas, mut transform, sprite,) in &mut query {
        if let Some(projectile) = projectile {
            transform.translation.y -= 70.0 * time.delta_seconds();
            
            unsafe {
                if transform.translation.y <= -HEIGHT/2.0 {
                    transform.translation.y = HEIGHT/2.0;
                }
            }
            
            // CHECK COLLISION TO PLAYER HERE !
            
        }
    }


}

#[derive(Component)]
struct Projectile {
    good: bool,
    size: Vec2,
}

#[derive(Component)]
struct Player {
    current: u32,
    max: u32,
}


fn setup( mut commands: Commands, asset_server: Res<AssetServer>, mut texture_assets: ResMut<TextureAssets>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>> ) {
    let TextureAssets { textures, sizes } = load_textures(asset_server.clone());
    texture_assets.textures = textures;
    texture_assets.sizes = sizes;

    let texture = asset_server.load("ferris_sprite_sheet.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(460.0, 246.0), 3, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 2 };
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            //transform: Transform::from_scale(Vec3::splat(0.5)),
            sprite: Sprite {
                custom_size: Some(PLAYER_SIZE),
                ..default()
            },
            texture,
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: animation_indices.first,
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player {
            current: 15,
            max: 35,
        },
    ));
    /*commands.spawn((
        SpriteBundle {
            transform: Transform::from_scale(Vec3::splat(0.5)),
            texture: asset_server.load("elephant.png"),
            ..default()
        },
        Projectile {
            good: true
        }
    ));*/
}

















/*
fn animate_sprite( time: Res<Time>, input: Res<ButtonInput<KeyCode>>, mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas, &mut Transform, &Sprite)> ) {
    for (indices, mut timer, mut atlas, mut transform, sprite) in &mut query {
        let oldX = transform.translation.x;
        let oldY = transform.translation.y;
        if input.pressed(KeyCode::KeyW) {
            transform.translation.y += 350.0 * time.delta_seconds();
        } if input.pressed(KeyCode::KeyS) {
            transform.translation.y -= 350.0 * time.delta_seconds();
        } if input.pressed(KeyCode::KeyD) {
            transform.translation.x += 350.0 * time.delta_seconds();
        } if input.pressed(KeyCode::KeyA) {
            transform.translation.x -= 350.0 * time.delta_seconds();
        }
        if oldX != transform.translation.x || oldY != transform.translation.y {
            //println!("Walked!");
            timer.tick(time.delta());
            if timer.just_finished() {
                atlas.index = if atlas.index == indices.last {
                    indices.first+1
                } else {
                    atlas.index + 1
                };
            };
        }
        else {
            atlas.index = indices.first;
        }
        println!("{}", atlas.index);
    }
}*/


/*use std::ptr::null;

use bevy::prelude::*;

const AJD: i32 = 5;


#[derive(Resource)]
struct PlayerSpriteSheet(Handle<TextureAtlasLayout>);

impl FromWorld for PlayerSpriteSheet {
    fn from_world(world: &mut World) -> Self {
        let texture_atlas = TextureAtlasLayout::from_grid(
            Vec2::new(100.0, 100.0), // The size of each image
            7, // The number of columns
            1, // The number of rows
            None, // Padding
            None // Offset
        );

        let mut texture_atlases = world.get_resource_mut::<Assets<TextureAtlasLayout>>().unwrap();
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        Self(texture_atlas_handle)
    }
}


//static mut WALK1: Handle<Image>;
//static mut WALK2: Handle<Image>;

fn update(
    mut characters: Query<(&mut Transform, &Sprite)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, spr) in &mut characters {
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
            println!("Walked!");
            //spr.texture = WALK1;
        }
cargo 
    }
}
//ferris_sprite_sheet.png
#[derive(Component)]
struct Person;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //commands.spawn(Camera2dBundle::default());
    commands.spawn(Camera2dBundle::default());

    let texture = asset_server.load("stand.png");
    //WALK1 = asset_server.load("walk1.png");
    //WALK2 = asset_server.load("walk2.png");



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
        .run();*/

    /*App::new().add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.3, 0.6)))
        .add_systems(Startup, setup)
        .add_systems(Update, hello_world)
        .run();*/
//}

    