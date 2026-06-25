//! Logic for handling UI interactions and dynamic layout updates.
//!
//! This module contains the Bevy systems that bridge the gap between user input
//! (clicks, slider drags) and the internal simulation state, as well as systems
//! that ensure the UI remains visually consistent across different window sizes.

use bevy::prelude::*;
use bevy_ui_widgets::{Slider, SliderRange, SliderThumb, SliderValue};
use std::f32::consts::PI;

use crate::components::*;
use crate::constants::{PRIMARY_EMISSIVE, NEUTRAL_TEXT, PANEL_WIDTH};
use crate::resources::*;

/// Handles interactions with the sequence control button (Play/Pause).
///
/// This system updates the [`SequenceState`] based on button clicks and
/// modifies the button's appearance (text and color) to provide visual
/// feedback on the current state of the simulation.
#[allow(clippy::type_complexity)]
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
                    background_color.0 = PRIMARY_EMISSIVE;
                } else {
                    text.0 = "Launch Sequence".to_string();
                    background_color.0 = Color::NONE;
                    // Reset accelerate counter when stopping, so it always starts fresh
                    // and doesn't immediately speed up upon restarting.
                    rhythm_state.accelerate_counter = 0;
                }
            }
            Interaction::Hovered => {
                background_color.0 = Color::srgba(0.886, 0.886, 0.886, 0.1);
            }
            Interaction::None => {
            }
        }
    }
}

/// Handles interactions with the curriculum toggle button.
///
/// Toggles the visibility of the curriculum documents and automatically pauses
/// the training sequence when opened.
#[allow(clippy::type_complexity)]
pub fn curriculum_toggle_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<CurriculumToggleButton>),
    >,
    mut text_query: Query<&mut Text>,
    mut curriculum_state: ResMut<CurriculumState>,
    mut sequence_state: ResMut<SequenceState>,
) {
    for (interaction, mut background_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                curriculum_state.is_visible = !curriculum_state.is_visible;
                if curriculum_state.is_visible {
                    text.0 = "Close Curriculum".to_string();
                    sequence_state.running = false;
                } else {
                    text.0 = "Open Curriculum".to_string();
                }
            }
            Interaction::Hovered => {
                background_color.0 = Color::srgba(0.886, 0.886, 0.886, 0.1);
            }
            Interaction::None => {
                background_color.0 = Color::NONE;
            }
        }
    }
}

/// Handles interactions with the sequence mode toggle button.
///
/// Switches the [`SequenceMode`] between Random and Ordered. It also resets
/// internal counters to ensure the transition between modes is clean and predictable.
#[allow(clippy::type_complexity)]
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
                background_color.0 = Color::srgba(0.886, 0.886, 0.886, 0.1);
            }
            Interaction::None => {
                background_color.0 = Color::NONE;
            }
        }
    }
}

/// Handles interactions with the rhythm mode toggle button.
///
/// Switches the [`RhythmMode`] between Constant and Accelerate. When switching,
/// it resets the acceleration progress to ensure the user has time to adjust
/// to the new behavior.
#[allow(clippy::type_complexity)]
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
                background_color.0 = Color::srgba(0.886, 0.886, 0.886, 0.1);
            }
            Interaction::None => {
                background_color.0 = Color::NONE;
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
    typography: Res<Typography>,
) {
    // Available space is the window minus the side panel.
    let available_width = window.resolution.width() - PANEL_WIDTH;
    let available_height = window.resolution.height();

    // Use minimum dimension for radius to ensure it fits comfortably with padding.
    let radius = available_width.min(available_height) / 2.0 * 0.8;
    // Scale font size linearly based on the radius to maintain visual proportions.
    let dynamic_font_size = radius * 0.25;

    let target_font_size = dynamic_font_size.max(10.0); // Ensure readability on small windows.

    for (index, mut transform, mut text_font) in &mut text_query {
        let i = index.0;
        // Calculate position on the circle. Start at 90 degrees (Top) and move clockwise.
        let angle = PI / 2.0 - (i as f32) * (PI / 4.0);

        let target_x = angle.cos() * radius - PANEL_WIDTH / 2.0;
        let target_y = angle.sin() * radius;
        let target_z = 1.0;

        // Only update translation if there is a meaningful change to prevent redundant change detection
        if (transform.translation.x - target_x).abs() > f32::EPSILON
            || (transform.translation.y - target_y).abs() > f32::EPSILON
            || (transform.translation.z - target_z).abs() > f32::EPSILON
        {
            transform.translation.x = target_x;
            transform.translation.y = target_y;
            transform.translation.z = target_z;
        }

        // Verify handle equality before assigning to avoid unnecessary atomic reference count increments
        if text_font.font != typography.space_grotesk {
            text_font.font = typography.space_grotesk.clone();
        }

        // Only update font size if it has changed
        if (text_font.font_size - target_font_size).abs() > f32::EPSILON {
            text_font.font_size = target_font_size;
        }
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
    sequence_state: Res<SequenceState>,
    mut gizmos: Gizmos,
) {
    // Don't show the arrow if the sequence is stopped.
    if !sequence_state.running {
        return;
    }

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

    // Use the primary emissive color for the tactical active signal.
    let color: Color = PRIMARY_EMISSIVE;

    // Draw the arrow multiple times with small offsets to simulate a thicker line,
    // as the default gizmo arrow does not support a thickness parameter.
    gizmos.arrow(start_pos.extend(1.0), current_end_pos.extend(1.0), color);
    
    let offset_v = (end_pos - start_pos).normalize().perp() * 2.0;
    gizmos.arrow((start_pos + offset_v).extend(1.0), (current_end_pos + offset_v).extend(1.0), color);
    gizmos.arrow((start_pos - offset_v).extend(1.0), (current_end_pos - offset_v).extend(1.0), color);
}

/// Synchronizes the visual appearance of target numbers with the [`CurrentNumber`].
///
/// This system updates the color of each target entity. If a target is the
/// currently active one, it is assigned the [`PRIMARY_EMISSIVE`] (glowing red);
/// otherwise, it receives the [`NEUTRAL_TEXT`] (white) at reduced opacity.
pub fn sync_target_visuals(
    current_number: Res<CurrentNumber>,
    sequence_state: Res<SequenceState>,
    mut query: Query<(&NumberIndex, &mut TextColor)>,
) {
    for (index, mut color) in &mut query {
        if sequence_state.running && index.0 == current_number.0 {
            color.0 = PRIMARY_EMISSIVE;
        } else {
            // Inactive targets use the neutral color at 100% opacity for maximum tactical clarity.
            color.0 = NEUTRAL_TEXT;
        }
    }
}

/// Handles interactions with the curriculum grade selection button.
///
/// Cycles through the available grades defined in [`crate::constants::CURRICULUM_MANIFEST`].
#[allow(clippy::type_complexity)]
pub fn curriculum_grade_system(
    mut interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<CurriculumGradeButton>),
    >,
    mut curriculum_state: ResMut<CurriculumState>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            let manifest = &crate::constants::CURRICULUM_MANIFEST;
            let mut grades: Vec<&&str> = manifest.keys().collect();
            grades.sort(); // Consistent order

            if grades.is_empty() {
                return;
            }

            if let Some(current) = &curriculum_state.selected_grade {
                if let Some(pos) = grades.iter().position(|&&g| g == current) {
                    let next_pos = (pos + 1) % grades.len();
                    let next_grade = grades[next_pos].to_string();
                    
                    // Update grade and reset document to the first one available for that grade
                    if let Some(docs) = manifest.get(next_grade.as_str()) {
                        if !docs.is_empty() {
                            curriculum_state.selected_grade = Some(next_grade);
                            curriculum_state.selected_document = Some(docs[0].0.to_string());
                        }
                    }
                }
            }
        }
    }
}

/// Handles interactions with the curriculum document selection button.
///
/// Cycles through the available documents for the currently selected grade.
#[allow(clippy::type_complexity)]
pub fn curriculum_document_system(
    mut interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<CurriculumDocumentButton>),
    >,
    mut curriculum_state: ResMut<CurriculumState>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            if let Some(grade) = &curriculum_state.selected_grade {
                if let Some(docs) = crate::constants::CURRICULUM_MANIFEST.get(grade.as_str()) {
                    if docs.is_empty() {
                        return;
                    }

                    if let Some(current_doc) = &curriculum_state.selected_document {
                        if let Some(pos) = docs.iter().position(|(d, _)| d == current_doc) {
                            let next_pos = (pos + 1) % docs.len();
                            curriculum_state.selected_document = Some(docs[next_pos].0.to_string());
                        }
                    }
                }
            }
        }
    }
}

/// Handles interactions with curriculum page navigation buttons (Prev/Next).
#[allow(clippy::type_complexity)]
pub fn curriculum_page_button_system(
    mut prev_query: Query<&Interaction, (Changed<Interaction>, With<CurriculumPrevPageButton>)>,
    mut next_query: Query<&Interaction, (Changed<Interaction>, With<CurriculumNextPageButton>)>,
    mut curriculum_state: ResMut<CurriculumState>,
) {
    if !curriculum_state.is_visible || curriculum_state.pages.is_empty() {
        return;
    }

    for interaction in &mut prev_query {
        if *interaction == Interaction::Pressed {
            if curriculum_state.current_page > 0 {
                curriculum_state.current_page -= 1;
            }
        }
    }

    for interaction in &mut next_query {
        if *interaction == Interaction::Pressed {
            if curriculum_state.current_page < curriculum_state.pages.len().saturating_sub(1) {
                curriculum_state.current_page += 1;
            }
        }
    }
}

/// Synchronizes the text labels of curriculum UI elements with the current state.
pub fn sync_curriculum_ui_labels(
    curriculum_state: Res<CurriculumState>,
    mut query: Query<(&mut Text, Option<&ParentButton<CurriculumGradeButton>>, Option<&ParentButton<CurriculumDocumentButton>>, Option<&CurriculumPageText>)>,
) {
    if !curriculum_state.is_changed() {
        return;
    }

    for (mut text, grade_btn, doc_btn, page_txt) in &mut query {
        if grade_btn.is_some() {
            if let Some(grade) = &curriculum_state.selected_grade {
                text.0 = format!("Grade: {}", grade);
            }
        } else if doc_btn.is_some() {
            if let Some(doc) = &curriculum_state.selected_document {
                text.0 = format!("Doc: {}", doc);
            }
        } else if page_txt.is_some() {
            if curriculum_state.pages.is_empty() {
                 text.0 = "Page: -- / --".to_string();
            } else {
                 text.0 = format!("Page: {} / {}", curriculum_state.current_page + 1, curriculum_state.pages.len());
            }
        }
    }
}

/// Helper trait to find text within a button's children.
/// In this project's UI structure, buttons have a single Text child.
#[derive(Component)]
pub struct ParentButton<T: Component>(pub std::marker::PhantomData<T>);
