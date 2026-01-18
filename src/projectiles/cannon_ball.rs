use crate::movable::Movable;
use crate::projectiles::Projectile;
use bevy::prelude::*;

/// Spawns a cannon ball projectile using mesh information from this module
pub fn spawn_cannon_ball_projectile(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    _asset_server: &Res<AssetServer>,
    _scene_spawner: &mut ResMut<SceneSpawner>,
    position: Vec3,
    velocity: Vec3,
    rotation: Quat,
) {
    let projectile_mesh = meshes.add(Sphere::new(0.03));
    let projectile_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 1.0, 1.0), // Cyan projectile
        emissive: Color::srgb(1.0, 1.0, 0.0).into(),
        ..default()
    });

    commands.spawn((
        Projectile::default(),
        Movable::with_velocity(velocity, 1.0),
        Mesh3d(projectile_mesh),
        MeshMaterial3d(projectile_material),
        Transform {
            translation: position,
            rotation,
            scale: Vec3::ONE,
        },
    ));
}
