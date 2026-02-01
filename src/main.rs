use bevy::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

const PANEL_WIDTH: f32 = 300.0;
const CIRCLE_RADIUS: f32 = 300.0;

#[derive(Component)]
struct NumberIndex(u8);

#[derive(Resource)]
struct HighlightTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(HighlightTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .add_systems(Startup, setup)
        .add_systems(Update, highlight_system)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn((
        Camera2d::default(),
        Transform::from_xyz(PANEL_WIDTH / 2.0, 0.0, 0.0),
    ));

    // UI Root Node
    commands.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::FlexEnd, // Push children (panel) to the right
        ..default()
    })
    .with_children(|parent| {
        // Configuration Panel
        parent.spawn((
            Node {
                width: Val::Px(PANEL_WIDTH),
                height: Val::Percent(100.0),
                border: UiRect::left(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            BorderColor::from(Color::WHITE),
        ));
    });

    // Circle of Numbers
    let text_font = TextFont {
        font_size: 70.0,
        ..default()
    };
    let text_color = TextColor(Color::WHITE);

    for i in 1..=8 {
        // 1 at top (PI/2), clockwise decrement by 45 degrees (PI/4)
        let angle = PI / 2.0 - (i as f32 - 1.0) * (PI / 4.0);
        
        let x = angle.cos() * CIRCLE_RADIUS;
        let y = angle.sin() * CIRCLE_RADIUS;

        commands.spawn((
            Text2d::new(format!("{i}")),
            text_font.clone(),
            text_color,
            Transform::from_xyz(x, y, 0.0),
            NumberIndex(i),
        ));
    }
}

fn highlight_system(
    time: Res<Time>,
    mut timer: ResMut<HighlightTimer>,
    mut query: Query<(&NumberIndex, &mut TextColor)>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::rng();
        let target = rng.random_range(1..=8);

        for (index, mut color) in &mut query {
            if index.0 == target {
                color.0 = Color::srgb(5.0, 0.0, 0.0); // Red highlight with high intensity for bloom
            } else {
                color.0 = Color::WHITE;
            }
        }
    }
}
