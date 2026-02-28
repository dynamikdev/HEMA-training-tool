use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

const PANEL_WIDTH: f32 = 300.0;
const CIRCLE_RADIUS: f32 = 300.0;
const LABELS: [u8; 8] = [7, 1, 5, 3, 8, 4, 6, 2];

#[derive(Component)]
struct NumberIndex(u8);

#[derive(Resource)]
struct HighlightTimer(Timer);

#[derive(Resource)]
struct CurrentNumber(u8);

#[derive(Default, PartialEq, Clone, Copy)]
enum SequenceMode {
    #[default]
    Random,
    Ordered,
}

#[derive(Resource, Default)]
struct SequenceState {
    running: bool,
    mode: SequenceMode,
    current_ordered_value: u8, // 1 to 7
}

#[derive(Component)]
struct SequenceControlButton;

#[derive(Component)]
struct SequenceModeButton;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(HighlightTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )))
        .insert_resource(CurrentNumber(1))
        .insert_resource(SequenceState {
            running: false,
            mode: SequenceMode::Random,
            current_ordered_value: 0,
        })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                highlight_system,
                sequence_control_button_system,
                mode_toggle_system,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn((
        Camera2d::default(),
        Tonemapping::TonyMcMapface,
        Bloom::default(),
        Transform::from_xyz(PANEL_WIDTH / 2.0, 0.0, 0.0),
    ));

    // UI Root Node
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::FlexEnd,
            ..default()
        })
        .with_children(|parent| {
            // Configuration Panel
            parent
                .spawn((
                    Node {
                        width: Val::Px(PANEL_WIDTH),
                        height: Val::Percent(100.0),
                        border: UiRect::left(Val::Px(2.0)),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    BorderColor::all(Color::WHITE),
                ))
                .with_children(|parent| {
                    // Panel Title
                    parent.spawn((
                        Text::new("Settings"),
                        TextFont {
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        Node {
                            margin: UiRect::bottom(Val::Px(20.0)),
                            ..default()
                        },
                    ));

                    // Mode Toggle Button
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(65.0),
                                border: UiRect::all(Val::Px(2.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::bottom(Val::Px(10.0)),
                                ..default()
                            },
                            BorderColor::all(Color::WHITE),
                            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                            SequenceModeButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Mode: Random"),
                                TextFont {
                                    font_size: 25.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                            ));
                        });

                    // Sequence Control Button
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(65.0),
                                border: UiRect::all(Val::Px(2.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::bottom(Val::Px(10.0)),
                                ..default()
                            },
                            BorderColor::all(Color::WHITE),
                            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                            SequenceControlButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Launch Sequence"),
                                TextFont {
                                    font_size: 30.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                            ));
                        });
                });
        });

    // Circle of Numbers
    let text_font = TextFont {
        font_size: 70.0,
        ..default()
    };
    let text_color = TextColor(Color::WHITE);
    for i in 0..=7 {
        let angle = PI / 2.0 - (i as f32) * (PI / 4.0);
        let x = angle.cos() * CIRCLE_RADIUS;
        let y = angle.sin() * CIRCLE_RADIUS;

        commands.spawn((
            Text2d::new(format!("{}", LABELS[i as usize])),
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
    mut highlighted_number: ResMut<CurrentNumber>,
    mut query: Query<(&NumberIndex, &mut TextColor)>,
    mut sequence_state: ResMut<SequenceState>,
) {
    if !sequence_state.running {
        return;
    }

    if timer.0.tick(time.delta()).just_finished() {
        let target_index = match sequence_state.mode {
            SequenceMode::Random => {
                let mut rng = rand::rng();
                let mut target = rng.random_range(0..=7);
                while target == highlighted_number.0 {
                    target = rng.random_range(0..=7);
                }
                target
            }
            SequenceMode::Ordered => {
                // Cycle values 1 to 8
                sequence_state.current_ordered_value = (sequence_state.current_ordered_value % 8) + 1;
                // Find index of this value in LABELS
                LABELS
                    .iter()
                    .position(|&l| l == sequence_state.current_ordered_value)
                    .unwrap_or(0) as u8
            }
        };

        highlighted_number.0 = target_index;
        for (index, mut color) in &mut query {
            if index.0 == target_index {
                color.0 = Color::srgb(5.0, 0.0, 0.0);
            } else {
                color.0 = Color::WHITE;
            }
        }
    }
}

fn sequence_control_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<SequenceControlButton>),
    >,
    mut text_query: Query<&mut Text>,
    mut sequence_state: ResMut<SequenceState>,
) {
    for (interaction, mut background_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                sequence_state.running = !sequence_state.running;
                if sequence_state.running {
                    text.0 = "Stop Sequence".to_string();
                    background_color.0 = Color::srgb(0.35, 0.15, 0.15);
                } else {
                    text.0 = "Launch Sequence".to_string();
                    background_color.0 = Color::srgb(0.15, 0.15, 0.15);
                }
            }
            Interaction::Hovered => {
                background_color.0 = Color::srgb(0.25, 0.25, 0.25);
            }
            Interaction::None => {
                if sequence_state.running {
                    background_color.0 = Color::srgb(0.3, 0.1, 0.1);
                } else {
                    background_color.0 = Color::srgb(0.15, 0.15, 0.15);
                }
            }
        }
    }
}

fn mode_toggle_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<SequenceModeButton>),
    >,
    mut text_query: Query<&mut Text>,
    mut sequence_state: ResMut<SequenceState>,
) {
    for (interaction, mut background_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                sequence_state.mode = match sequence_state.mode {
                    SequenceMode::Random => SequenceMode::Ordered,
                    SequenceMode::Ordered => SequenceMode::Random,
                };
                // Reset ordered progress when switching modes or just to be safe
                sequence_state.current_ordered_value = 0;

                match sequence_state.mode {
                    SequenceMode::Random => {
                        text.0 = "Mode: Random".to_string();
                    }
                    SequenceMode::Ordered => {
                        text.0 = "Mode: Ordered".to_string();
                    }
                }
            }
            Interaction::Hovered => {
                background_color.0 = Color::srgb(0.25, 0.25, 0.25);
            }
            Interaction::None => {
                background_color.0 = Color::srgb(0.15, 0.15, 0.15);
            }
        }
    }
}
