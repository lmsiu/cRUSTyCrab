use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_systems(Startup, setup)
        .add_systems(Update, update_player)
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
    /*mut projectile_query: Query<
        (
            Entity,
            //&mut Transform,
            &Sprite,
            &Projectile,
        ),
    >,*/
) {
    for (player, projectile, indices, mut timer, mut atlas, mut transform, sprite,
    ) in &mut query
    {
        if let (Some(player), Some(indices), Some(mut timer), Some(mut atlas)) = (player, indices, timer.as_mut(), atlas.as_mut())   // if this is a PLAYER
        {
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
            } else {
                atlas.index = indices.first;
            }

// ======= PROJECTILE MOVEMENT AND COLLISION DETECTION CODE ========
            
        }
    }
}

#[derive(Component)]
struct Projectile {
    wtf: u32,
}

#[derive(Component)]
struct Player {
    current: u32,
    max: u32,
}


fn setup( mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>> ) {
    let texture = asset_server.load("ferris_sprite_sheet.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(460.0, 307.0), 3, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 2 };
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_scale(Vec3::splat(0.5)),
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
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_scale(Vec3::splat(0.5)),
            texture: asset_server.load("elephant.png"),
            ..default()
        },
        Projectile {
            wtf: 10
        }
    ));
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

    