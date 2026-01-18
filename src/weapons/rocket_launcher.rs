use crate::projectiles::spawn_rocket_projectile;
use crate::weapons::weapon::Weapon;
use bevy::prelude::*;

/// Creates a rocket launcher weapon with specified position offset
/// Rocket launchers have no mesh and fire rockets with longer cooldown
pub fn create_rocket_launcher(weapon_position: Vec3) -> Weapon {
    Weapon::new()
        .with_fire_cooldown(1.0) // Longer cooldown than cannon
        .with_projectile_spawner(spawn_rocket_projectile)
        // No mesh spawner - rocket launcher has no visible mesh
        .with_weapon_position_offset(weapon_position)
        .with_projectile_spawn_offset(Vec3::new(0.2, 0.0, 0.0)) // Slightly further forward than cannon
}
