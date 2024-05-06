use bevy::prelude::*;
use bevy::window::{Window, WindowResolution, WindowPlugin};

#[derive(Component)]
struct Collider;

#[derive(Bundle)]
struct FloorBundle {
    sprite_bundle:SpriteBundle,
    collider:Collider,
}

impl FloorBundle {
    fn new(m_texture:Handle<Image>) -> FloorBundle {
        FloorBundle {
            sprite_bundle: SpriteBundle {
                texture: m_texture,
                ..default()
            },
            collider: Collider,
        }
    }
}

pub fn get_autorunner_game() {
    return App::new()
        /*.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Autorunner!".to_string(),
                resolution: WindowResolution::new(800., 600.),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))*/
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server:Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(FloorBundle::new(asset_server.load("m_brick.png")));

}