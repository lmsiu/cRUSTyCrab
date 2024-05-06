use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy::render::render_resource::Texture;
use bevy::window::{PresentMode, WindowResized};
use rand::Rng;

use std::time::{Duration, SystemTime, UNIX_EPOCH};


static mut WIDTH: f32 = 1280.0;
static mut HEIGHT: f32 = 720.0;

const PROJECTILE_SCALE: f32 = 0.35;

const PROJECTILE_SPEED: f32 = 400.0;

const PLAYER_SIZE: Vec2 = Vec2::new(0.5*460.0, 0.5*246.0);

const PLAYER_SPEED: f32 = 550.0; 

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
    let mut sizes = vec![Vec2::new(300.0 * s, 185.0 * s),   // food1     -  shrimp
                                    Vec2::new(300.0 * s, 113.0 * s),   // food2     -  fish
                                    Vec2::new(300.0 * s, 261.0 * s),   // food3     -  algae
                                    Vec2::new(300.0 * s, 153.0 * s),   // food4     -  worm
                                    Vec2::new(300.0 * s, 237.0 * s),   // food5     -  sea lettuce?
                                    Vec2::new(300.0 * s, 179.0 * s),   // harmful1  -  rock
                                    Vec2::new(300.0 * s, 300.0 * s),   // harmful2  -  fishing hook
                                    Vec2::new(255.0 * s, 300.0 * s)];  // harmful3  -  tire

    TextureAssets { textures, sizes }
}

fn seconds_since_epoch() -> u64 {
    let now = SystemTime::now();
    let duration_since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    duration_since_epoch.as_secs()
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Seafood Scramble".into(),
                resolution: (1280., 720.).into(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        //.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_systems(Startup, setup)
        .insert_resource(TextureAssets { textures: Vec::new(), sizes: Vec::new() })
        .add_systems(
            Update,
            (
                spawn_projectile.run_if(on_timer(Duration::from_millis(500))),
            )
        )
        .add_systems(Update, update_player)
        .add_systems(Update, resize_notificator)
        .add_systems(Update, (update_health_text, update_score_text))
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

fn check_collision(pos1: Vec2, size1: Vec2, pos2: Vec2, size2: Vec2) -> bool {
    let left1   = pos1.x - size1.x/2.0;
    let right1  = pos1.x + size1.x/2.0;
    let top1    = pos1.y + size1.y/2.0;
    let bottom1 = pos1.y - size1.y/2.0;

    let left2   = pos2.x - size2.x/2.0;
    let right2  = pos2.x + size2.x/2.0;
    let top2    = pos2.y + size2.y/2.0;
    let bottom2 = pos2.y - size2.y/2.0;
    
    // If one rectangle is on the left side of the other
    if right1 <= left2 || right2 <= left1 {
        return false;
    }

    // If one rectangle is above the other
    if bottom1 >= top2 || bottom2 >= top1 {
        return false;
    }

    println!("collide! {} {}", pos1, pos2); 
    // If they are neither, then they must be colliding.
    return true;
}

static mut player_health: i32 = 3;
static mut player_score: i32 = 0;

fn update_health_text(mut query: Query<&mut Text, With<HealthText>>,) {
    for mut text in &mut query {
        unsafe {
            text.sections[0].value = format!("Health: {player_health:.2}");
        }
        
    }
}
fn update_score_text(mut query: Query<&mut Text, With<ScoreText>>,) {
    for mut text in &mut query {
        unsafe {
            text.sections[0].value = format!("Score: {player_score:.2}");
        }
        
    }
}

fn spawn_projectile(mut commands: Commands, texture_assets: Res<TextureAssets>,) {
    let index = rand::thread_rng().gen_range(0..texture_assets.textures.len());

    let size = texture_assets.sizes[index];

    let screen_height;
    let screen_width;
    unsafe {
        screen_height = HEIGHT;
        screen_width = WIDTH;
    }
    let x = rand::thread_rng().gen_range(-screen_width/2.0 + size.x/2.0..screen_width/2.0 - size.x/2.0);
    let y = screen_height/2.0 + size.y/2.0;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(size),
                ..default()
            },
            transform: Transform::from_xyz(x, y, 2.0),
            //transform: Transform::from_scale(Vec3::splat(0.5)),
            texture: texture_assets.textures[index].clone_weak(),
            ..default()
        },
        Projectile {
            good: index < 5, // if index is less than 5, then its one of the food. otherwise its bad. 
            size: size,
        },
    ));
}


fn update_player(
    mut commands: Commands,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (
            Entity,

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
    let mut player_pos = Vec2::ZERO;
    for (entity, player, projectile, indices, mut timer, mut atlas, mut transform, sprite,) in &mut query
    {
        if let (Some(player), Some(indices), Some(mut timer), Some(mut atlas)) = (player, indices, timer.as_mut(), atlas.as_mut())   // if this is a PLAYER
        {
            let left_bound;
            let right_bound;
            unsafe {
                if player_health <= 0 {
                    transform.rotation = Quat::from_rotation_z(std::f32::consts::PI); // Rotate 180 degrees;
                    return;
                }
                transform.translation.y = -HEIGHT/2.0 + PLAYER_SIZE.y;

                left_bound = -WIDTH/2.0 + PLAYER_SIZE.x/2.0;
                right_bound = -left_bound;
            }

            player_pos = Vec2::new(transform.translation.x, transform.translation.y);
// ======= FERRIS WALKING CODE ========
            let old_x = transform.translation.x;
            
            //if input.pressed(KeyCode::KeyW) {
            //    transform.translation.y += PLAYER_SPEED * time.delta_seconds();
            //}
            //if input.pressed(KeyCode::KeyS) {
            //    transform.translation.y -= PLAYER_SPEED * time.delta_seconds();
            //}
            if input.pressed(KeyCode::KeyD) {
                transform.translation.x += PLAYER_SPEED * time.delta_seconds();
            }
            if input.pressed(KeyCode::KeyA) {
                transform.translation.x -= PLAYER_SPEED * time.delta_seconds();
            }
            
            transform.translation.x = f32::max(left_bound, f32::min(right_bound, transform.translation.x)); // lock it inside the bounds.
            

            if old_x != transform.translation.x {
                timer.tick(time.delta());
                if timer.just_finished() {
                    atlas.index = if atlas.index == indices.last {
                        indices.first + 1
                    } else {
                        atlas.index + 1
                    };
                }
                //println!("X = {} Y = {}", transform.translation.x, transform.translation.y);
            } else {
                atlas.index = indices.first;
            }
        }
    }
// ======= PROJECTILE MOVEMENT AND COLLISION DETECTION CODE ========
    for (entity, player, projectile, indices, mut timer, mut atlas, mut transform, sprite,) in &mut query {
        if let Some(projectile) = projectile {
            transform.translation.y -= PROJECTILE_SPEED * time.delta_seconds();
            
            let screen_height;
            unsafe {
                screen_height = HEIGHT; 
            }

            // CHECK COLLISION TO PLAYER 
            let projectile_pos = Vec2::new(transform.translation.x, transform.translation.y);
            if check_collision(player_pos, PLAYER_SIZE, projectile_pos, projectile.size) {
                unsafe {
                    if projectile.good {
                        player_score+=1;
                    } else {
                        player_health-=1;
                    }
                }
                commands.entity(entity).despawn();
            }
            else if transform.translation.y + projectile.size.y/2.0 <= -screen_height/2.0 { // if the top of the projectile is below the bottom of the screen
                commands.entity(entity).despawn();
            }
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
    wtf: u32,
}

#[derive(Component)]
struct HealthText;

#[derive(Component)]
struct ScoreText;



fn setup( mut commands: Commands, asset_server: Res<AssetServer>, mut texture_assets: ResMut<TextureAssets>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>> ) {
    let TextureAssets { textures, sizes } = load_textures(asset_server.clone());
    texture_assets.textures = textures;
    texture_assets.sizes = sizes;

    let background_image = asset_server.load("background.png");
    let texture = asset_server.load("ferris_sprite_sheet.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(460.0, 246.0), 3, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 2 };
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: background_image,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
    commands.spawn((
        SpriteBundle {
            //transform: Transform::from_scale(Vec3::splat(0.5)),
            sprite: Sprite {
                custom_size: Some(PLAYER_SIZE),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -720.0/2.0 + PLAYER_SIZE.y, 1.0),
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
            wtf: 0 
        },
    ));
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            "Health: 3",
            TextStyle {
                font_size: 30.0,
                ..default()
            },
        ) // Set the justification of the Text
        .with_text_justify(JustifyText::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(30.0),
            left: Val::Px(30.0),
            ..default()
        }),
        HealthText,
    ));
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            "Score: 0",
            TextStyle {
                font_size: 30.0,
                ..default()
            },
        ) // Set the justification of the Text
        .with_text_justify(JustifyText::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(30.0),
            right: Val::Px(30.0),
            ..default()
        }),
        ScoreText,
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

    