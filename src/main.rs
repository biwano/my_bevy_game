mod movable;
mod projectiles;
mod ship;
mod starfield;
mod target;
mod weapons;

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

use movable::MovablePlugin;
use projectiles::ProjectilePlugin;
use ship::ShipPlugin;
use starfield::{move_stars, rotate_skybox, setup_starfield};
use target::{
    check_projectile_target_collisions, despawn_dead_targets, despawn_out_of_bounds_targets,
    setup_targets, update_target_colors,
};
use weapons::WeaponPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MovablePlugin)
        .add_plugins(ShipPlugin)
        .add_plugins(WeaponPlugin)
        .add_plugins(ProjectilePlugin)
        .add_systems(Startup, (setup, setup_starfield, setup_targets))
        .add_systems(
            Update,
            (
                move_stars,
                rotate_skybox,
                check_projectile_target_collisions,
                update_target_colors,
                despawn_dead_targets,
                despawn_out_of_bounds_targets,
            ),
        )
        .run();
}

/// set up a simple 3D scene
fn setup(mut commands: Commands) {
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
