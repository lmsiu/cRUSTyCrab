use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::window::{Window, WindowResolution, WindowPlugin};

#[derive(Bundle)]
struct FloorBundle {
    sprite_bundle:SpriteBundle,
    tiling:ImageScaleMode,
}

#[derive(Component)]
struct Player(i32);

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

pub fn get_autorunner_game() {
    return App::new()
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
        .add_systems(Update, controls)
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
}

fn controls(input:Res<ButtonInput<KeyCode>>, time:Res<Time>,mut query:Query<(&mut Velocity, &mut Player)>) {
    let (mut player, whatever)= query.single_mut();
    if input.just_pressed(KeyCode::Space) {
        player.linvel = Vec2::new(0., 300.);
    }
}