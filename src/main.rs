mod ship;
mod starfield;
mod projectile;
mod target;

use bevy::{
    color::palettes::css::*,
    pbr::wireframe::{NoWireframe, Wireframe, WireframeColor, WireframeConfig, WireframePlugin},
    prelude::*,
    render::{
        RenderPlugin,
        render_resource::WgpuFeatures,
        settings::{RenderCreation, WgpuSettings},
    },
};

use ship::{move_spaceship, setup_ship, spawn_projectile};
use starfield::{setup_starfield, move_stars, rotate_skybox};
use projectile::{move_projectiles, despawn_out_of_bounds_projectiles};
use target::{setup_targets, check_projectile_target_collisions};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, setup_ship, setup_starfield, setup_targets))
        .add_systems(Update, (move_spaceship, move_stars, rotate_skybox, spawn_projectile, move_projectiles, despawn_out_of_bounds_projectiles, check_projectile_target_collisions))
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
) {
    // Set background to pure black
    commands.insert_resource(ClearColor(Color::BLACK));

    // directional light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, -0.5, 0.0)),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Text used to show controls
    commands.spawn((
        Text::default(),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));
}
