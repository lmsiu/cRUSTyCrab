use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::window::{Window, WindowResolution, WindowPlugin};
use std::time::Duration;
use rand::Rng;

#[derive(Bundle)]
struct FloorBundle {
    sprite_bundle:SpriteBundle,
    tiling:ImageScaleMode,
}


#[derive(Bundle)]
struct ObstacleBundle {
    sprite_bundle:SpriteBundle,
    rigidbody:RigidBody,
    collider:Collider,
    velocity:Velocity,
}

#[derive(Component)]
struct Player(i32);

#[derive(Resource)]
struct RockTime {
    timer:Timer,
}

#[derive(Component)]
struct Ui;

#[derive(Resource)]
struct GameScore {
    timer:Timer,
    score:u32,
    game_running:bool,
}

impl GameScore {
    fn get_score(self) -> u32 {
        return self.score;
    }

    fn increment_score(mut self) {
        self.score += 1;
    }
}

impl FloorBundle {
    fn new(m_texture:Handle<Image>) -> FloorBundle {
        FloorBundle {
            sprite_bundle: SpriteBundle {
                texture: m_texture,
                transform: Transform::from_xyz(0.,-268.,0.),
                sprite: Sprite {
                    custom_size:Some(Vec2::new(800.,64.)),
                    ..default()
                },
                ..default()
            },
            tiling:ImageScaleMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 1.,
            },
        }
    }
}

impl ObstacleBundle {
    fn new(m_texture:Handle<Image>) -> ObstacleBundle {
        ObstacleBundle {
            sprite_bundle: SpriteBundle {
                texture: m_texture,
                transform: Transform::from_xyz(500.,-190.,0.)
                    .with_scale(Vec3::new(0.25,0.25,1.)),
                ..default()
            },
            rigidbody: RigidBody::KinematicVelocityBased,
            velocity: Velocity {
                linvel: Vec2::new(-rand::thread_rng().gen_range(200..230) as f32,0.),
                ..default()
            },
            collider: Collider::ball(105.),
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Autorunner!".to_string(),
                resolution: WindowResolution::new(800., 600.),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(10.0)) // Physics plugin
        .add_plugins(RapierDebugRenderPlugin::default()) // Debug plugin
        .add_systems(Startup, setup)
        .add_systems(Update, (bevy::window::close_on_esc, controls, throw_rocks, score_handler, scoreboard_updater, death_handler, death_handler2))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server:Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    // floor
    commands.spawn(FloorBundle::new(asset_server.load("m_brick.png")))
        .insert(Collider::cuboid(400.,32.));
    
    // player
    commands.spawn(RigidBody::Dynamic)
        .insert(Collider::ball(130.0))
        .insert(KinematicCharacterController::default())
        .insert(SpriteBundle {
            texture: asset_server.load("rustacean-flat-happy.png"),
            transform: Transform::from_scale(Vec3::new(0.25,0.25,1.)).with_translation(Vec3::new(-200., -200., 0.)),
            ..Default::default()
        })
        .insert(GravityScale(5.0))
        .insert(Velocity {
            linvel:Vec2::new(0.,0.),
            ..Default::default()
        })
        .insert(Player(0));
    
    
    commands.insert_resource(RockTime{
        timer:Timer::from_seconds(1.7, TimerMode::Repeating),
    });

    commands.insert_resource(GameScore{
        timer:Timer::from_seconds(1., TimerMode::Repeating),
        score:0,
        game_running:true,
    });

    commands.spawn((
        Ui,
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle{
                    font_size:40.,
                    color:Color::rgb(0.5, 0.5, 1.0),
                    ..default()
                }
            ),
            TextSection::from_style(
                TextStyle{
                    font_size:40.,
                    color:Color::rgb(0.5, 0.5, 1.0),
                    ..default()
                }
            ),
        ]),
    ));
}

fn controls(input:Res<ButtonInput<KeyCode>>,mut query:Query<(&mut Velocity, &mut Player)>) {
    let (mut player, _whatever)= query.single_mut();
    if input.just_pressed(KeyCode::Space) {
        player.linvel = Vec2::new(0., 300.);
    }
}

fn throw_rocks(mut commands:Commands, time: Res<Time>, mut rock_time: ResMut<RockTime>,
    asset_server:Res<AssetServer>, mut score_res: ResMut<GameScore>) {
    rock_time.timer.tick(time.delta());

    if rock_time.timer.just_finished() && score_res.game_running {
        commands.spawn(ObstacleBundle::new(asset_server.load("harmful1.png")))
            .insert(Sensor);
    }
}

fn score_handler(mut commands:Commands, time: Res<Time>, mut score_res: ResMut<GameScore>) {
    score_res.timer.tick(time.delta());

    if score_res.timer.just_finished() && score_res.game_running {
        score_res.score += 1;
    }
}

fn scoreboard_updater(score_res: Res<GameScore>, mut query: Query<&mut Text, With<Ui>>) {
    let mut text = query.single_mut();
    text.sections[1].value = score_res.score.to_string();
}

fn death_handler(mut score_res:ResMut<GameScore>, rapier_context:Res<RapierContext>, mut query:Query<Entity, With<Player>>) {
    let entity = query.single_mut();
    for (collider1, collider2, intersecting) in rapier_context.intersection_pairs_with(entity) {
        if intersecting {
            println!("There was an intersection!");
            score_res.game_running = false;
        }
    }
}

fn death_handler2(mut score_res:ResMut<GameScore>, mut query:Query<&mut Transform, With<Player>>) {
    let mut transform = query.single_mut();
    if !score_res.game_running {
        transform.rotation = Quat::from_rotation_z(std::f32::consts::PI);
    }
}