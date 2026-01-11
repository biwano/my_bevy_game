use bevy::prelude::*;
use crate::projectile::Projectile;

#[derive(Component)]
pub struct Target;

pub fn setup_targets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create a cube mesh for targets
    let cube_mesh = meshes.add(Cuboid::new(0.5, 0.5, 0.5));
    
    // Create a red material for targets
    let target_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.0, 0.0), // Red target
        ..default()
    });
    
    // Spawn 5 cubes at different positions (all at z=0)
    let positions = [
        Vec3::new(3.0, 0.0, 0.0),
        Vec3::new(3.0, 2.0, 0.0),
        Vec3::new(3.0, -2.0, 0.0),
        Vec3::new(3.0, 1.0, 0.0),
        Vec3::new(3.0, -1.0, 0.0),
    ];
    
    for position in positions.iter() {
        commands.spawn((
            Target,
            Mesh3d(cube_mesh.clone()),
            MeshMaterial3d(target_material.clone()),
            Transform::from_translation(*position),
        ));
    }
}

pub fn check_projectile_target_collisions(
    mut commands: Commands,
    projectiles: Query<(Entity, &Transform), With<Projectile>>,
    targets: Query<(Entity, &Transform), With<Target>>,
) {
    let collision_distance = 0.5; // Collision detection distance
    
    for (projectile_entity, projectile_transform) in projectiles.iter() {
        let projectile_pos = projectile_transform.translation;
        
        for (target_entity, target_transform) in targets.iter() {
            let target_pos = target_transform.translation;
            let distance = projectile_pos.distance(target_pos);
            
            // If projectile is close enough to target, despawn both
            if distance < collision_distance {
                commands.entity(projectile_entity).despawn();
                commands.entity(target_entity).despawn();
                break; // Only hit one target per projectile
            }
        }
    }
}

