//! Logic for handling UI interactions and dynamic layout updates.
//!
//! This module contains the Bevy systems that bridge the gap between user input
//! (clicks, slider drags) and the internal simulation state, as well as systems
//! that ensure the UI remains visually consistent across different window sizes.

use bevy::prelude::*;
use bevy_ui_widgets::{Slider, SliderRange, SliderThumb, SliderValue};
use std::f32::consts::PI;

use crate::components::*;
use crate::constants::PANEL_WIDTH;
use crate::resources::*;

/// Handles interactions with the sequence control button (Play/Pause).
///
/// This system updates the [`SequenceState`] based on button clicks and
/// modifies the button's appearance (text and color) to provide visual
/// feedback on the current state of the simulation.
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
                    // Reset accelerate counter when stopping, so it always starts fresh
                    // and doesn't immediately speed up upon restarting.
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
///
/// Switches the [`SequenceMode`] between Random and Ordered. It also resets
/// internal counters to ensure the transition between modes is clean and predictable.
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
                // Reset ordered progress when switching modes to start from the beginning.
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
///
/// Switches the [`RhythmMode`] between Constant and Accelerate. When switching,
/// it resets the acceleration progress to ensure the user has time to adjust
/// to the new behavior.
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

                // Reset counter when toggling modes to provide a fresh start for acceleration.
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

/// Manages the visual state of the slider widget's thumb.
///
/// Since the slider widget is custom-built, this system manually calculates
/// the thumb's position based on the current [`SliderValue`] and [`SliderRange`].
pub fn style_slider_system(
    mut thumb_nodes: Query<&mut Node, With<SliderThumb>>,
    slider_query: Query<(&SliderValue, &SliderRange, &ComputedNode, &Children), With<Slider>>,
) {
    for (value, range, track_computed, children) in &slider_query {
        let track_size = track_computed.size();
        let is_vertical = track_size.y > track_size.x;

        let thumb_extent = 16.0; // Fixed size defined during spawn.
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

/// Automatically adjusts the circular layout of targets to fit the window.
///
/// This system ensures that the training targets are always centered in the
/// available screen space (accounting for the side panel) and that their
/// size remains legible regardless of window dimensions.
pub fn update_circle_layout(
    window: Single<&bevy::window::Window, With<bevy::window::PrimaryWindow>>,
    mut text_query: Query<(&NumberIndex, &mut Transform, &mut TextFont)>,
) {
    // Available space is the window minus the side panel.
    let available_width = window.resolution.width() - PANEL_WIDTH;
    let available_height = window.resolution.height();

    // Use minimum dimension for radius to ensure it fits comfortably with padding.
    let radius = available_width.min(available_height) / 2.0 * 0.8;
    // Scale font size linearly based on the radius to maintain visual proportions.
    let dynamic_font_size = radius * 0.25;

    for (index, mut transform, mut text_font) in &mut text_query {
        let i = index.0;
        // Calculate position on the circle. Start at 90 degrees (Top) and move clockwise.
        let angle = PI / 2.0 - (i as f32) * (PI / 4.0);

        let x = angle.cos() * radius;
        let y = angle.sin() * radius;

        // Offset X to center the circle in the area to the left of the side panel.
        transform.translation.x = x - PANEL_WIDTH / 2.0;
        transform.translation.y = y;
        text_font.font_size = dynamic_font_size.max(10.0); // Ensure readability on small windows.
    }
}

/// Updates the simulation's rhythm whenever the user interacts with the slider.
///
/// This system provides a "grid-snapping" behavior to make the rhythm selection
/// more user-friendly and ensures that the internal timer is immediately
/// updated to reflect the new desired duration.
pub fn update_rhythm_from_slider(
    slider_query: Query<&SliderValue, Changed<SliderValue>>,
    mut rhythm_state: ResMut<RhythmState>,
    mut highlight_timer: ResMut<HighlightTimer>,
    mut text_query: Query<&mut Text, With<RhythmText>>,
) {
    for slider_val in &slider_query {
        // Snap to grid of 0.1s for easier user selection.
        let value = (slider_val.0 * 10.0).round() / 10.0;

        // Only update if there is a significant change to avoid jitter.
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

/// Renders the glowing guide arrow between the active and opposite targets.
///
/// This system uses Bevy's gizmo API to draw a high-intensity red arrow that
/// triggers the Bloom effect. The arrow's length is animated based on the
/// current [`ArrowAnimationState`].
pub fn render_glowing_arrow(
    window: Single<&bevy::window::Window, With<bevy::window::PrimaryWindow>>,
    current_number: Res<CurrentNumber>,
    arrow_target: Res<ArrowTarget>,
    animation_state: Res<ArrowAnimationState>,
    mut gizmos: Gizmos,
) {
    // Only render if we have a valid target index (it might be None during setup).
    let target_idx = match arrow_target.0 {
        Some(idx) => idx,
        None => return,
    };

    // Calculate dimensions similar to update_circle_layout to ensure alignment.
    let available_width = window.resolution.width() - PANEL_WIDTH;
    let available_height = window.resolution.height();
    let radius = available_width.min(available_height) / 2.0 * 0.8;
    let offset_x = -PANEL_WIDTH / 2.0;

    let start_idx = current_number.0;
    let end_idx = target_idx;

    // Convert indices to polar coordinates (angles) and then to screen space positions.
    let start_angle = PI / 2.0 - (start_idx as f32) * (PI / 4.0);
    let end_angle = PI / 2.0 - (end_idx as f32) * (PI / 4.0);

    let start_pos = Vec2::new(
        start_angle.cos() * radius + offset_x,
        start_angle.sin() * radius,
    );
    let end_pos = Vec2::new(
        end_angle.cos() * radius + offset_x,
        end_angle.sin() * radius,
    );

    // Linearly interpolate the arrow's endpoint based on animation progress.
    let current_end_pos = start_pos.lerp(end_pos, animation_state.progress);

    // Use a high-intensity red color (exceeding 1.0 in some channels) to trigger
    // the Bloom glow effect on the primary camera.
    let color = LinearRgba::new(10.0, 0.0, 0.0, 1.0);

    gizmos.arrow_2d(start_pos, current_end_pos, color);
}
