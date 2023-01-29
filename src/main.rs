use bevy::prelude::*;
use bevy::utils::FloatOrd;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::{
    prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use physics::{PhysicsBundle, PhysicsPlugin};

mod bullet;
mod physics;
mod target;
mod tower;

pub use bullet::*;
pub use target::*;
pub use tower::*;

pub const WIDTH: f32 = 1080.0;
pub const HEIGHT: f32 = 720.0;
pub const PI: f32 = 3.41;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_basic_scene)
        .add_startup_system(asset_loading)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WIDTH,
                height: HEIGHT,
                title: "Bevy Tower Game".to_string(),
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(TowerPlugin)
        .add_plugin(BulletPlugin)
        .add_plugin(TargetPlugin)
        .run();
}

// spawns the camera in
fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert(Name::new("Ground"));

    // Tower
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(Tower {
            bullet_offset: Vec3 {
                x: 0.0,
                y: 0.8,
                z: 0.0,
            },
            shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        })
        .insert(Name::new("Tower"));

    // Target 1
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
            material: materials.add(Color::rgb(0.87, 0.74, 0.82).into()),
            transform: Transform::from_xyz(-2.0, 0.2, 1.5),
            ..default()
        })
        .insert(Target { speed: 0.15 })
        .insert(Health { value: 3 })
        .insert(PhysicsBundle::moving_entity(Vec3::new(0.4, 0.4, 0.4)))
        .insert(Name::new("Target"));

    // Target 2
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
            material: materials.add(Color::rgb(0.87, 0.74, 0.82).into()),
            transform: Transform::from_xyz(-2.5, 0.2, 1.5),
            ..default()
        })
        .insert(Target { speed: 0.1 })
        .insert(Health { value: 3 })
        .insert(PhysicsBundle::moving_entity(Vec3::new(0.2, 0.2, 0.2)))
        .insert(Name::new("Target"));

    // Light
    commands
        .spawn(PointLightBundle {
            transform: Transform::from_xyz(2.0, 8.0, 2.0),
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        })
        .insert(Name::new("Light"));
}

#[derive(Resource)]
pub struct GameAssets {
    pub bullet_scene: Handle<Scene>,
}

pub fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        bullet_scene: assets.load("Bullet.glb#Scene0"),
    });
}
