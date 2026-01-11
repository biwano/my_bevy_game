use bevy::prelude::*;

#[derive(Component)]
pub struct Projectile;

pub fn move_projectiles(
    mut projectiles: Query<&mut Transform, With<Projectile>>,
    time: Res<Time>,
) {
    let speed = 10.0; // Speed of projectile movement
    
    for mut transform in projectiles.iter_mut() {
        // Move projectile from left to right (increase X)
        transform.translation.x += speed * time.delta_secs();
    }
}

pub fn despawn_out_of_bounds_projectiles(
    mut commands: Commands,
    projectiles: Query<(Entity, &Transform), With<Projectile>>,
) {
    let boundary = 50.0; // Distance from origin before despawning
    
    for (entity, transform) in projectiles.iter() {
        let pos = transform.translation;
        // Despawn if projectile is too far in any direction
        if pos.x.abs() > boundary || pos.y.abs() > boundary || pos.z.abs() > boundary {
            commands.entity(entity).despawn();
        }
    }
}

