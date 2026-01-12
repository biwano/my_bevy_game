use bevy::prelude::*;
use crate::projectile::Projectile;
use crate::movable::Movable;

#[derive(Component)]
pub struct Weapon {
    pub fire_cooldown_duration: f32,
    pub cooldown_timer: f32,
}

impl Weapon {
    pub fn new(fire_cooldown_duration: f32) -> Self {
        Self {
            fire_cooldown_duration,
            cooldown_timer: 0.0,
        }
    }
    
    pub fn can_fire(&self) -> bool {
        self.cooldown_timer <= 0.0
    }
    
    pub fn start_cooldown(&mut self) {
        self.cooldown_timer = self.fire_cooldown_duration;
    }
}

pub fn update_weapon_cooldowns(
    mut weapons: Query<&mut Weapon>,
    time: Res<Time>,
) {
    for mut weapon in weapons.iter_mut() {
        if weapon.cooldown_timer > 0.0 {
            weapon.cooldown_timer -= time.delta_secs();
            if weapon.cooldown_timer < 0.0 {
                weapon.cooldown_timer = 0.0;
            }
        }
    }
}

pub fn activate_weapon(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut weapons: Query<&mut Weapon>,
    spaceship_entity: Res<crate::ship::SpaceshipEntity>,
    transforms: Query<&Transform>,
    movables: Query<&Movable>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Check if space is pressed (can be held down)
    if keyboard_input.pressed(KeyCode::Space) {
        // Get the weapon attached to the spaceship
        if let Ok(mut weapon) = weapons.get_mut(spaceship_entity.0) {
            // Check if weapon can fire (cooldown has passed)
            if weapon.can_fire() {
                // Get spaceship position and velocity
                if let (Ok(spaceship_transform), Ok(ship_movable)) = (transforms.get(spaceship_entity.0), movables.get(spaceship_entity.0)) {
                    // Create projectile based on weapon type
                    // For now, using default projectile type
                    let projectile_mesh = meshes.add(Sphere::new(0.03));
                    let projectile_material = materials.add(StandardMaterial {
                        base_color: Color::srgb(0.0, 1.0, 1.0), // Cyan projectile
                        emissive: Color::srgb(1.0, 1.0, 0.0).into(),
                        ..default()
                    });
                    
                    // Calculate projectile velocity: ship velocity + forward velocity (to the right)
                    let forward_speed = 10.0;
                    let projectile_velocity = ship_movable.velocity + Vec3::new(forward_speed, 0.0, 0.0);
                    
                    // Spawn projectile at spaceship position with inherited velocity
                    commands.spawn((
                        Projectile::default(),
                        Movable::with_velocity(projectile_velocity, 1.0),
                        Mesh3d(projectile_mesh),
                        MeshMaterial3d(projectile_material),
                        Transform::from_translation(spaceship_transform.translation),
                    ));
                    
                    // Start cooldown
                    weapon.start_cooldown();
                }
            }
        }
    }
}

