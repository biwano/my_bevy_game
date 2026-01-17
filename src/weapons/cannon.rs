use bevy::prelude::*;
use crate::weapons::weapon::Weapon;
use crate::projectiles::spawn_cannon_ball;

/// Creates a cannon weapon with default settings
pub fn create_cannon() -> Weapon {
    Weapon::new()
        .with_fire_cooldown(0.5)
        .with_projectile_spawner(spawn_cannon_ball)
        .with_mesh_spawner(spawn_cannon_mesh)
}

/// Spawns the cannon mesh as a child of the given parent entity
pub fn spawn_cannon_mesh(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    scene_spawner: &mut ResMut<SceneSpawner>,
    parent_entity: Entity,
    translation: Vec3,
) {
    let weapon_mesh_handle = asset_server.load("models/cannon.glb#Scene0");
    let weapon_mesh_entity = commands.spawn((
        Transform {
            translation, // Position relative to ship
            rotation: Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2), // Rotate 90 degrees around Z axis
            scale: Vec3::splat(10.0), // Scale to match ship scale
        },
    )).id();
    scene_spawner.spawn_as_child(weapon_mesh_handle, weapon_mesh_entity);
    // Attach weapon mesh as a child of the parent
    commands.entity(parent_entity).add_child(weapon_mesh_entity);
}
