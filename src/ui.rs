//! UI implementation for the HEMA Training Tool, including the configuration panel and circular target layout.

use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;
use bevy_ui_widgets::{Slider, SliderPlugin, SliderRange, SliderStep, SliderThumb, SliderValue};
use std::f32::consts::PI;

use crate::components::*;
use crate::constants::{CIRCLE_RADIUS, LABELS, PANEL_WIDTH};
use crate::resources::*;

/// Plugin that initializes and manages the training tool's user interface.
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SliderPlugin)
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    sequence_control_button_system,
                    mode_toggle_system,
                    rhythm_mode_toggle_system,
                    style_slider_system,
                    update_rhythm_from_slider,
                    update_circle_layout,
                ),
            );
    }
}

/// Initial setup for the UI, including the camera, the settings panel, and the target numbers.
fn setup(mut commands: Commands) {
    // 2D Camera with Bloom effect for the high-intensity highlight.
    commands.spawn((
        Camera2d::default(),
        Tonemapping::TonyMcMapface,
        Bloom::default(),
        // Offset the camera so (0,0) is centered in the space remaining after the panel.
        Transform::from_xyz(PANEL_WIDTH / 2.0, 0.0, 0.0),
    ));

    // UI Root Node: Full screen container.
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::FlexEnd, // Push children to the right side.
            ..default()
        })
        .with_children(|parent| {
            // Configuration Panel: Fixed-width side panel on the right.
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

                    // Mode Toggle Button: Switches between Random and Ordered sequences.
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

                    // Sequence Control Button: Starts/Stops the training sequence.
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

                    // Rhythm Mode Toggle Button: Constant vs Accelerate timing.
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(45.0),
                                border: UiRect::all(Val::Px(2.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::bottom(Val::Px(10.0)),
                                ..default()
                            },
                            BorderColor::all(Color::WHITE),
                            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                            RhythmModeButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Rhythm: Constant"),
                                TextFont {
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                            ));
                        });

                    // Rhythm current value label.
                    parent.spawn((
                        Text::new("Rhythm: 1.0s"),
                        TextFont {
                            font_size: 25.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        Node {
                            margin: UiRect::bottom(Val::Px(5.0)),
                            ..default()
                        },
                        RhythmText,
                    ));

                    // Rhythm Slider widget (vertical).
                    parent
                        .spawn((
                            Node {
                                width: Val::Px(24.0),
                                height: Val::Px(200.0),
                                align_self: AlignSelf::Center,
                                margin: UiRect::bottom(Val::Px(20.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                            BorderColor::all(Color::WHITE),
                            Interaction::default(),
                            Slider::default(),
                            SliderValue(1.0),
                            SliderRange::new(0.5, 3.0),
                            SliderStep(0.1),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Px(16.0),
                                    position_type: PositionType::Absolute,
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.8, 0.8, 0.8)),
                                SliderThumb,
                            ));
                        });
                });
        });

    // Circle of Numbers (initial spawn placeholder).
    let text_font = TextFont {
        font_size: 70.0,
        ..default()
    };
    let text_color = TextColor(Color::WHITE);
    for i in 0..=7 {
        // Calculation centered at (0,0), later refined by update_circle_layout.
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

/// Handles interactions with the start/stop sequence button.
fn sequence_control_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<SequenceControlButton>),
    >,
    mut text_query: Query<&mut Text>,
    mut sequence_state: ResMut<SequenceState>,
    mut rhythm_state: ResMut<RhythmState>,
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
                    // Reset accelerate counter when stopping, so it always starts fresh.
                    rhythm_state.accelerate_counter = 0;
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

/// Handles interactions with the sequence mode toggle button.
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
                // Reset ordered progress when switching modes or just to be safe.
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

/// Handles interactions with the rhythm mode toggle button.
fn rhythm_mode_toggle_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<RhythmModeButton>),
    >,
    mut text_query: Query<&mut Text>,
    mut rhythm_state: ResMut<RhythmState>,
) {
    for (interaction, mut background_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                rhythm_state.mode = match rhythm_state.mode {
                    RhythmMode::Constant => RhythmMode::Accelerate,
                    RhythmMode::Accelerate => RhythmMode::Constant,
                };

                // Reset counter when toggling modes.
                rhythm_state.accelerate_counter = 0;

                match rhythm_state.mode {
                    RhythmMode::Constant => {
                        text.0 = "Rhythm: Constant".to_string();
                    }
                    RhythmMode::Accelerate => {
                        text.0 = "Rhythm: Accelerate".to_string();
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

/// Visual styling system for the slider thumb position.
fn style_slider_system(
    mut thumb_nodes: Query<&mut Node, With<SliderThumb>>,
    slider_query: Query<(&SliderValue, &SliderRange, &ComputedNode, &Children), With<Slider>>,
) {
    for (value, range, track_computed, children) in &slider_query {
        let track_size = track_computed.size();
        let is_vertical = track_size.y > track_size.x;

        let thumb_extent = 16.0; // Fixed size given in spawn.
        let track_extent = if is_vertical {
            track_size.y
        } else {
            track_size.x
        };

        if track_extent > thumb_extent {
            let percent =
                ((value.0 - range.start()) / (range.end() - range.start())).clamp(0.0, 1.0);
            let max_pos = 1.0_f32 - (thumb_extent / track_extent);

            for &child in children {
                if let Ok(mut thumb_node) = thumb_nodes.get_mut(child) {
                    if is_vertical {
                        thumb_node.bottom = Val::Percent(percent * max_pos * 100.0);
                        thumb_node.left = Val::Auto;
                    } else {
                        thumb_node.left = Val::Percent(percent * max_pos * 100.0);
                        thumb_node.bottom = Val::Auto;
                    }
                }
            }
        }
    }
}

/// Dynamically updates the circular layout of numbers to fit the window and panel.
fn update_circle_layout(
    window: Single<&bevy::window::Window, With<bevy::window::PrimaryWindow>>,
    mut text_query: Query<(&NumberIndex, &mut Transform, &mut TextFont)>,
) {
    // Available space is the window minus the side panel.
    let available_width = window.resolution.width() - PANEL_WIDTH;
    let available_height = window.resolution.height();

    // Use minimum dimension for radius to ensure it perfectly fits inside remaining space, with a little padding.
    let radius = available_width.min(available_height) / 2.0 * 0.8;
    // Scale font size linearly based on radius.
    let dynamic_font_size = radius * 0.25;

    for (index, mut transform, mut text_font) in &mut text_query {
        let i = index.0;
        // Calculate position on the circle. Start at 90 degrees (Top) and move clockwise.
        let angle = PI / 2.0 - (i as f32) * (PI / 4.0);

        let x = angle.cos() * radius;
        let y = angle.sin() * radius;

        transform.translation.x = x;
        transform.translation.y = y;
        text_font.font_size = dynamic_font_size.max(10.0); // Ensure readability on small windows.
    }
}

/// Syncs the simulation rhythm state with the UI slider value.
fn update_rhythm_from_slider(
    slider_query: Query<&SliderValue, Changed<SliderValue>>,
    mut rhythm_state: ResMut<RhythmState>,
    mut highlight_timer: ResMut<HighlightTimer>,
    mut text_query: Query<&mut Text, With<RhythmText>>,
) {
    for slider_val in &slider_query {
        // Snap to grid of 0.1s.
        let value = (slider_val.0 * 10.0).round() / 10.0;

        if rhythm_state.duration != value {
            rhythm_state.duration = value;
            for mut text in &mut text_query {
                text.0 = format!("Rhythm: {:.1}s", value);
            }

            highlight_timer
                .0
                .set_duration(std::time::Duration::from_secs_f32(value));
        }
    }
}
