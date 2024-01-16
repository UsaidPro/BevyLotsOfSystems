//! A simple 3D scene with light shining over a cube sitting on a plane.

use std::f32::consts::PI;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rand::random;
use repeat_macro::simulations;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};


fn main() {
    let mut app = App::new();
    app
        .add_plugins((DefaultPlugins, RapierPhysicsPlugin::<NoUserData>::default()))//, RapierDebugRenderPlugin::default()))
        // .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup_graphics);
    app.add_plugins((LogDiagnosticsPlugin::default(), FrameTimeDiagnosticsPlugin::default()));
    simulations!(app);
    app.run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera to see the render.
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn setup_physics<const T: usize>(mut commands: Commands,
                 mut meshes: ResMut<Assets<Mesh>>,
                 mut materials: ResMut<Assets<StandardMaterial>>) {
    /* Create the ground. */
    let offset = 10.0 * (T as f32);
    commands.spawn(
        PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0, ..default()})),
                material: materials.add(Color::WHITE.into()),
                transform: Transform::from_xyz(0.0 + offset, 0.0, 0.0)
                    .with_rotation(Quat::from_euler(EulerRot::XYZ,
                                                    (random::<f32>() - 0.5) * 5.0 * PI/180.0,
                                                    0.0,
                                                    (random::<f32>() - 0.5) * 5.0 * PI/180.0)),
                ..Default::default()
            }
    )
        .insert(Name::new(format!("Board{}", T)))
        .insert(RigidBody::KinematicPositionBased)
        .insert(LockedAxes::TRANSLATION_LOCKED | LockedAxes::ROTATION_LOCKED_Y)
        .insert(Collider::cuboid(5.0, 0.1, 5.0))
        .insert(IsBoard)
        .insert(Simulation::<T>);

    // light
    // commands.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 1500.0,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(4.0 + offset, 8.0, 4.0),
    //     ..default()
    // })
    //     .insert(Name::new(format!("Light{}", T)));

    /* Create the bouncing ball. */
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 0.5, ..default() })),
            material: materials.add(Color::rgb_u8(124, 144, 255).into()),
            transform: Transform::from_xyz(0.0 + offset, 4.5, 0.0),
            ..default()
        })
        .insert(Name::new(format!("Ball{}", T)))
        .insert(RigidBody::Dynamic)
        .insert(Velocity::default())
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(Simulation::<T>);
}

/// Determines if ball falls below threshold and environment must be reset
fn must_reset<const T: usize>(query: Query<&Transform, (With<Simulation<T>>, With<Restitution>)>) -> bool {
    let transform = query.single();
    if transform.translation.y < -2.0 {
        return true;
    }
    return false;
}

/// Resets ball position and changes board to random angle
fn reset_simulation<const T: usize>(mut query: Query<(&mut Transform, &RigidBody, Option<&mut Velocity>), With<Simulation<T>>>) {
    for (mut transform, _rigid_body, mut is_ball) in query.iter_mut() {
        if let Some(mut ball_vel) = is_ball {
            // Current entity is the ball, reset height
            transform.translation = Vec3::new(0.0 + 10.0 * (T as f32), 4.5, 0.0);
            ball_vel.linvel = Vec3::new(0.0, 0.0, 0.0);
            ball_vel.angvel = Vec3::new(0.0, 0.0, 0.0);
        }
        else {
            transform.rotation = Quat::from_euler(EulerRot::XYZ,
                                                  (random::<f32>() - 0.5) * 5.0 * PI/180.0,
                                                  0.0,
                                                  (random::<f32>() - 0.5) * 5.0 * PI/180.0);
        }
    }
}

/// Randomly moves board as environment "action"
fn board_movement<const T: usize>(mut query: Query<&mut Transform, (With<Simulation<T>>, With<IsBoard>)>) {
    let mut transform = query.single_mut();
    let mut rotation = Vec3::new(0.0, 0.0, 0.0);
    let input_dir: i8 = random::<i8>() % 4;
    if input_dir == 0 && transform.rotation.x < 0.1 {
        rotation.x = 1.0;
    }
    if input_dir == 1 && transform.rotation.x > -0.1 {
        rotation.x = -1.0;
    }
    if input_dir == 2 && transform.rotation.z > -0.1 {
        rotation.z = -1.0;
    }
    if input_dir == 3 && transform.rotation.z < 0.1 {
        rotation.z = 1.0;
    }
    rotation *= PI/180.0;
    transform.rotate(Quat::from_euler(EulerRot::XYZ, rotation.x, rotation.y, rotation.z));
}

/// Tag component indicating if entity is a board
#[derive(Component)]
struct IsBoard;

/// Marker tag component indicating which environment entities are part of
#[derive(Component)]
#[component(storage = "SparseSet")]
struct Simulation<const T: usize>;

