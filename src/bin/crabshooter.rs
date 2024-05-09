use bevy::{prelude::*};
use bevy::utils::default;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Score{score: 0})
        .insert_resource(Health{health: 3})
        .add_systems(Startup,setup_game)
        .add_systems(Update, (move_player, shoot_projectile, move_projectiles, destroy_projectiles, kill_enemy,
                              hurt_player, kill_player, projectile_collision, enemy_projectile, move_enemy_projectiles,
                              destroy_enemy_projectiles, spawn_enemy))
        .add_systems(Update, (update_health_text, update_score_text))
        .insert_resource(ProjectileTimer(Timer::from_seconds(0.5, TimerMode::Once)))
        .insert_resource(EnemyProjectileTimer(Timer::from_seconds(2.0, TimerMode::Once)))
        .insert_resource(EnemySpawnTimer(Timer::from_seconds(3.0, TimerMode::Once)))
        .run();
}
// player object
#[derive(Component)]
struct Player;
// score tracker
#[derive(Resource, Clone, Copy)]
struct Score{
    score: i32,
}
// health tracker
#[derive(Resource, Clone, Copy)]
struct Health{
    health: i32,
}

// player projectile
#[derive(Component)]
struct Projectile;
// enemy projectile
#[derive(Component)]
struct EnemyProjectile;
// Timer used to limit player shooting every frame per second
#[derive(Resource)]
struct ProjectileTimer(Timer);
// enemy projectile interval
#[derive(Resource)]
struct EnemyProjectileTimer(Timer);
// enemy spawn interval
#[derive(Resource)]
struct EnemySpawnTimer(Timer);
// The Enemy object
#[derive(Component)]
struct Enemy;
// text for health display
#[derive(Component)]
struct HealthText;
// text for score display
#[derive(Component)]
struct ScoreText;

const PLAYER_SIZE: Vec2 = Vec2::new(0.25*460.0, 0.25*307.0);
const PLAYER_STARTING_POSITION: Vec3 = Vec3::new(0.0, -200.0, 1.0);
const ENEMY_STARTING_HEIGHT: f32 = 200.0;
const ENEMY_SIZE: Vec2 = Vec2::new(0.25*315.0, 0.25*250.0);

const WIDTH: f32 = 1280.0;

fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("textures/oceanbg.png"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
    // Spawn Player in initial position
    commands.spawn((SpriteBundle {
                texture: asset_server.load("textures/rustacean-flat-happy.png"),
                transform: Transform {
                    translation: PLAYER_STARTING_POSITION,
                    ..default()
                },
                sprite: Sprite {
                    custom_size: Some(PLAYER_SIZE),
                    //color: PADDLE_COLOR,
                    ..default()
                },
                ..default()
            },
            Player));
    // Spawn first enemy
    let random = rand::thread_rng().gen_range(-200.0..=200.0);
    commands.spawn((SpriteBundle {
        texture: asset_server.load("textures/corro.png"),
        transform: Transform {
            translation: Vec3::new(random-random%10.0, ENEMY_STARTING_HEIGHT, 1.0),
            ..default()
        },
        sprite: Sprite {
            custom_size: Some(ENEMY_SIZE),
            //color: PADDLE_COLOR,
            ..default()
        },
        ..default()
    }, Enemy));
    // healht text
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
    // score text
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

}

// Defines the amount of time that should elapse between each physics step
// in this case, 60fps
const TIME_STEP: f32 = 1.0 / 60.0;
const PLAYER_SPEED: f32 = 300.0;
fn move_player(input: Res<ButtonInput<KeyCode>>,
               mut query: Query<&mut Transform, With<Player>>,){
    let left_bound = -WIDTH/2.0 + PLAYER_SIZE.x/2.0;
    let right_bound = -left_bound;
    let mut player_transform = query.single_mut();
    let mut direction = 0.0;
    if input.pressed(KeyCode::ArrowLeft) {
        direction -= 1.0;
    }
    if input.pressed(KeyCode::ArrowRight) {
        //println!("[KEYBOARD] Pressed right");
        direction += 1.0;
    }
    let new_player_position = player_transform.translation.x + direction * PLAYER_SPEED * TIME_STEP;
    player_transform.translation.x = new_player_position;
    player_transform.translation.x = f32::max(left_bound, f32::min(right_bound, player_transform.translation.x));
}
// spawn an enemy when enemy timer ticks down
fn spawn_enemy(mut commands: Commands,
               asset_server: Res<AssetServer>,
               time: Res<Time>,
               mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, ){
    if enemy_spawn_timer.0.tick(time.delta()).finished() {
        enemy_spawn_timer.0.reset();
        let random = rand::thread_rng().gen_range(-200.0..=200.0);
        commands.spawn((SpriteBundle {
            texture: asset_server.load("textures/corro.png"),
            transform: Transform {
                translation: Vec3::new(random - random%10.0, ENEMY_STARTING_HEIGHT, 1.0),
                ..default()
            },
            sprite: Sprite {
                custom_size: Some(ENEMY_SIZE),
                ..default()
            },
            ..default()
        }, Enemy));
    }
}

const PROJECTILE_SIZE: Vec2 = Vec2::new(0.25*67.0, 0.25*90.0);
const ENEMY_PROJECTILE_SIZE: Vec2 = Vec2::new(0.25*70.0, 0.25*126.0);
const PROJECTILE_SPEED: f32 = 250.0;
const ENEMY_PROJECTILE_SPEED: f32 = 175.0;

fn shoot_projectile(
    time: Res<Time>,
    mut projectile_timer: ResMut<ProjectileTimer>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&Transform, With<Player>>,
) {
    let player_transform = query.single_mut();

    if input.pressed(KeyCode::Space) {
        // Check if player is allowed to shoot based on internal timer
        // We have to "tick" the timer to update it with the latest time
        //println!("{}", projectile_timer.0.elapsed_secs());
        if projectile_timer.0.tick(time.delta()).finished() {
            // Reset the timer
            projectile_timer.0.reset();
            // Spawn projectile
            commands.spawn((SpriteBundle {
                texture: asset_server.load("textures/player_projectile.png"),
                transform: Transform {
                    translation: player_transform.translation,
                    ..default()
                },
                sprite: Sprite {
                    custom_size: Some(PROJECTILE_SIZE),
                    ..default()
                },
                ..default()
            }, Projectile));

        }
    }
}
// spawn enemy projectiles
fn enemy_projectile(
    time: Res<Time>,
    mut enemy_projectile_timer: ResMut<EnemyProjectileTimer>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&Transform, With <Enemy>>,
) {
    for enemy_transform in query.iter() {
        if enemy_projectile_timer.0.tick(time.delta()).finished() {
            // Reset the timer
            enemy_projectile_timer.0.reset();
            // Spawn projectile
            commands.spawn((SpriteBundle {
                texture: asset_server.load("textures/enemy_projectile.png"),
                transform: Transform {
                    translation: enemy_transform.translation,
                    ..default()
                },
                sprite: Sprite {
                    custom_size: Some(ENEMY_PROJECTILE_SIZE),
                    ..default()
                },
                ..default()
            }, EnemyProjectile));
        }
    }
}
// move player projectiles
fn move_projectiles(mut query: Query<&mut Transform, With<Projectile>>) {
    for mut projectile_transform in &mut query {
        // Calculate the new horizontal player position based on player input
        let new_projectile_position = projectile_transform.translation.y + PROJECTILE_SPEED * TIME_STEP;
        projectile_transform.translation.y = new_projectile_position;
    }
}

fn move_enemy_projectiles(mut query: Query<&mut Transform, With<EnemyProjectile>>) {
    for mut projectile_transform in &mut query {
        let new_projectile_position = projectile_transform.translation.y - ENEMY_PROJECTILE_SPEED * TIME_STEP;
        projectile_transform.translation.y = new_projectile_position;
    }
}
// destroy projectiles when they go off screen
fn destroy_projectiles(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Projectile>>,
) {
    for (projectile_entity, projectile_transform) in &query {
        if projectile_transform.translation.y > 350.0 {
            commands.entity(projectile_entity).despawn();
        }
    }
}

fn destroy_enemy_projectiles(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<EnemyProjectile>>,
) {
    for (projectile_entity, projectile_transform) in &query {
        if projectile_transform.translation.y < -350.0 {
            commands.entity(projectile_entity).despawn();
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

    // If they are neither, then they must be colliding.
    return true;
}
// check for collision between player projectile and enemy
fn kill_enemy(mut commands: Commands,
              mut score: ResMut<Score>,
              enemy_query: Query<(Entity, &mut Transform), (With<Enemy>, Without<Projectile>)>,
              projectile_query: Query<&Transform, (With<Projectile>, Without<Enemy>)>) {
    for (entity, enemy_transform) in enemy_query.iter() {
        for projectile_transform in projectile_query.iter(){
            let projectile_pos = Vec2::new(projectile_transform.translation.x, projectile_transform.translation.y);
            let enemy_pos = Vec2::new(enemy_transform.translation.x, enemy_transform.translation.y);
            if check_collision(projectile_pos, PROJECTILE_SIZE, enemy_pos, ENEMY_SIZE) {
                // despawn the projectile
                commands.entity(entity).despawn();
                // increase score
                score.score += 1;
            }
        }
    }
}
// check for collisions between enemy projectile and player
fn hurt_player(mut commands: Commands,
               mut health: ResMut<Health>,
               player_query: Query<&Transform, (With<Player>, Without<EnemyProjectile>)>,
               enemy_projectile_query: Query<(Entity, &Transform), (With<EnemyProjectile>, Without<Player>)>) {
    for player_transform in player_query.iter() {
        for (enemy_projectile_entity, enemy_projectile_transform) in enemy_projectile_query.iter(){
            let enemy_projectile_pos = Vec2::new(enemy_projectile_transform.translation.x, enemy_projectile_transform.translation.y);
            let player_pos = Vec2::new(player_transform.translation.x, player_transform.translation.y);
            if check_collision(enemy_projectile_pos, ENEMY_PROJECTILE_SIZE, player_pos, PLAYER_SIZE){
                // decrease health
                health.health -= 1;
                // despawn projectile
                commands.entity(enemy_projectile_entity).despawn();
            }
        }
    }
}
// despawn projectiles that collide with each other
fn projectile_collision(mut commands: Commands,
                        projectile_query: Query<(Entity, &Transform), (With<Projectile>, Without<EnemyProjectile>)>,
                        enemy_projectile_query: Query<(Entity, &Transform), (With<EnemyProjectile>, Without<Player>)>) {
    for (player_projectile_entity, player_projectile_transform) in projectile_query.iter() {
        for (enemy_projectile_entity, enemy_projectile_transform) in enemy_projectile_query.iter(){
            let enemy_projectile_pos = Vec2::new(enemy_projectile_transform.translation.x, enemy_projectile_transform.translation.y);
            let player_projectile_pos = Vec2::new(player_projectile_transform.translation.x, player_projectile_transform.translation.y);
            if check_collision(enemy_projectile_pos, ENEMY_PROJECTILE_SIZE, player_projectile_pos, PROJECTILE_SIZE){
                commands.entity(enemy_projectile_entity).despawn();
                commands.entity(player_projectile_entity).despawn();
            }
        }
    }
}
// when player reaches health 0, end the game
fn kill_player(mut commands: Commands,
               health: ResMut<Health>,
               mut player_query: Query<&mut Transform, With<Player>>,
               mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
               mut enemy_projectile_timer: ResMut<EnemyProjectileTimer>,
               enemy_projectile_query: Query<Entity, (With<EnemyProjectile>, Without<Player>, Without<Projectile>)>,
               player_projectile_query: Query<Entity, (With<Projectile>, Without<Player>, Without<EnemyProjectile>)>
) {
    if health.health < 1 {
        // pause enemy actions
        enemy_spawn_timer.0.pause();
        enemy_projectile_timer.0.pause();
        // flip sprite over
        for mut transform in player_query.iter_mut() {
            transform.rotation = Quat::from_rotation_z(std::f32::consts::PI); // Rotate 180 degrees;
        }
        // despawn enemies
        for entity in enemy_projectile_query.iter(){
            commands.entity(entity).despawn();
        }
        // despawn projectiles
        for entity in player_projectile_query.iter(){
            commands.entity(entity).despawn();
        }
        // create death message
        commands.spawn((
            // Create a TextBundle that has a Text with a single section.
            TextBundle::from_section(
                "You died!",
                TextStyle {
                    font_size: 100.0,
                    ..default()
                }
            ) // Set the justification of the Text
                .with_text_justify(JustifyText::Center)
                // Set the style of the TextBundle itself.
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(100.0),
                    left: Val::Px(50.0),
                    right: Val::Px(50.0),
                    ..default()
                }),
        ));
    }
}
// update health text
fn update_health_text(mut query: Query<&mut Text, With<HealthText>>,
                      health: ResMut<Health>,) {
    for mut text in &mut query {
        text.sections[0].value = format!("Health: {:.2?}", health.health);
    }
}
// update score text
fn update_score_text(mut query: Query<&mut Text, With<ScoreText>>,
                     score: ResMut<Score>,) {
    for mut text in &mut query {
        text.sections[0].value = format!("Score: {:.2?}", score.score);
    }
}
