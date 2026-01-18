use crate::movable::Movable;
use crate::projectiles::Projectile;
use bevy::prelude::*;

/// Spawns a rocket projectile using the rocket.glb mesh
pub fn spawn_rocket_projectile(
    commands: &mut Commands,
    _meshes: &mut ResMut<Assets<Mesh>>,
    _materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
    scene_spawner: &mut ResMut<SceneSpawner>,
    position: Vec3,
    velocity: Vec3,
) {
    let rocket_scene_handle = asset_server.load("models/projectiles/rocket.glb#Scene0");

    let rocket_entity = commands
        .spawn((
            Projectile { damage: 25.0 }, // Rockets do more damage than cannon balls
            Movable::with_velocity(velocity, 1.0),
            Transform::from_translation(position)
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)) // Rotate to face forward
                .with_scale(Vec3::splat(0.5)), // Scale down the rocket
        ))
        .id();

    // Spawn the rocket mesh as a child of the projectile entity
    scene_spawner.spawn_as_child(rocket_scene_handle, rocket_entity);
}
