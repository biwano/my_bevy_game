use bevy::prelude::*;

#[derive(Component)]
pub struct Movable {
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub damping: f32,
}

impl Movable {
    pub fn new(velocity: Vec3, acceleration: Vec3, damping: f32) -> Self {
        Self {
            velocity,
            acceleration,
            damping,
        }
    }
    
    pub fn zero(damping: f32) -> Self {
        Self {
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            damping,
        }
    }
    
    pub fn with_velocity(velocity: Vec3, damping: f32) -> Self {
        Self {
            velocity,
            acceleration: Vec3::ZERO,
            damping,
        }
    }
}

pub fn apply_acceleration_to_movable(
    mut movables: Query<&mut Movable>,
    time: Res<Time>,
) {
    for mut movable in movables.iter_mut() {
        // Store acceleration and damping before modifying velocity
        let acceleration = movable.acceleration;
        let damping = movable.damping;
        
        // Apply acceleration to velocity
        movable.velocity += acceleration * time.delta_secs();
        
        // Apply damping/friction
        movable.velocity *= damping;
    }
}

pub fn move_movable(
    mut movables: Query<(&mut Transform, &Movable)>,
    time: Res<Time>,
) {
    for (mut transform, movable) in movables.iter_mut() {
        // Update position based on velocity
        transform.translation += movable.velocity * time.delta_secs();
    }
}

pub struct MovablePlugin;

impl Plugin for MovablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            apply_acceleration_to_movable,
            move_movable,
        ));
    }
}

