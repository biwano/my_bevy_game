use bevy::prelude::*;

#[derive(Component)]
pub struct ScoreDisplay;

#[derive(Resource)]
pub struct PlayerScore {
    pub score: u32,
}

impl Default for PlayerScore {
    fn default() -> Self {
        Self { score: 0 }
    }
}

pub fn setup_hud(mut commands: Commands) {
    // Text used to show controls
    commands.spawn((
        Text::new("CONTROLS:\nZQSD - Move\nQ/E - Rotate\n1/2 - Switch Weapons\nSPACE - Fire"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(12),
            left: px(12),
            ..default()
        },
    ));

    // Score display
    commands.spawn((
        ScoreDisplay,
        Text::new("Score: 0"),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(12),
            right: px(12),
            ..default()
        },
    ));
}

pub fn update_score_display(
    player_score: Res<PlayerScore>,
    mut score_text_query: Query<&mut Text, With<ScoreDisplay>>,
) {
    for mut text in score_text_query.iter_mut() {
        *text = Text::new(format!("Score: {}", player_score.score));
    }
}

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerScore>()
            .add_systems(Startup, setup_hud)
            .add_systems(Update, update_score_display);
    }
}
