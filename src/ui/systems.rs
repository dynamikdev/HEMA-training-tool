use bevy::prelude::*;
use bevy_ui_widgets::{Slider, SliderRange, SliderThumb, SliderValue};
use std::f32::consts::PI;

use crate::components::*;
use crate::constants::PANEL_WIDTH;
use crate::resources::*;

/// Handles interactions with the start/stop sequence button.
pub fn sequence_control_button_system(
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
pub fn mode_toggle_system(
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
pub fn rhythm_mode_toggle_system(
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
pub fn style_slider_system(
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
pub fn update_circle_layout(
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

        // Offset X to center the circle in the remaining area to the left of the side panel.
        transform.translation.x = x - PANEL_WIDTH / 2.0;
        transform.translation.y = y;
        text_font.font_size = dynamic_font_size.max(10.0); // Ensure readability on small windows.
    }
}

/// Syncs the simulation rhythm state with the UI slider value.
pub fn update_rhythm_from_slider(
    slider_query: Query<&SliderValue, Changed<SliderValue>>,
    mut rhythm_state: ResMut<RhythmState>,
    mut highlight_timer: ResMut<HighlightTimer>,
    mut text_query: Query<&mut Text, With<RhythmText>>,
) {
    for slider_val in &slider_query {
        // Snap to grid of 0.1s.
        let value = (slider_val.0 * 10.0).round() / 10.0;

        if (rhythm_state.duration - value).abs() > 0.01 {
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
