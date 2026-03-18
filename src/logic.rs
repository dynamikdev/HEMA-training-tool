//! Core training simulation logic for target highlighting and rhythm control.

use bevy::prelude::*;
use bevy_ui_widgets::{Slider, SliderValue};
use rand::Rng;

use crate::components::*;
use crate::resources::*;

/// Plugin that handles the core training logic, such as the highlighting system.
pub struct TrainingPlugin;

impl Plugin for TrainingPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<TargetInputEvent>()
            .add_systems(Update, (
            update_sequence_logic,
            handle_target_input,
            handle_feedback,
            update_feedback_visuals,
            update_stats,
            sync_rhythm_timer,
            sync_target_visuals,
            sync_rhythm_ui
        ).chain());
    }
}

/// System that updates session statistics based on input events.
fn update_stats(
    mut message_reader: MessageReader<TargetInputEvent>,
    mut stats: ResMut<SessionStats>,
) {
    for event in message_reader.read() {
        if event.correct {
            stats.correct_count += 1;
        } else {
            stats.incorrect_count += 1;
        }
    }
}

/// System that handles visual feedback when a target is hit.
fn handle_feedback(
    mut message_reader: MessageReader<TargetInputEvent>,
    current_number: Res<CurrentNumber>,
    mut commands: Commands,
    target_query: Query<(Entity, &NumberIndex)>,
) {
    for event in message_reader.read() {
        // Find the target entity that corresponds to the active index.
        // Actually, for incorrect hits, should we flash the incorrect target or the active one?
        // Let's flash the active one for now, as it's the focal point.
        for (entity, index) in &target_query {
            if index.0 == current_number.0 {
                let color = if event.correct {
                    Color::srgb(0.0, 5.0, 0.0) // Bright Green
                } else {
                    Color::srgb(5.0, 5.0, 0.0) // Bright Yellow/Orange for incorrect? Or just Red.
                };
                
                commands.entity(entity).insert((
                    FeedbackTimer(Timer::from_seconds(0.3, TimerMode::Once)),
                    TextColor(color),
                ));
            }
        }
    }
}

/// System that updates targets under feedback and removes the FeedbackTimer when finished.
fn update_feedback_visuals(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut FeedbackTimer, &mut TextColor, &NumberIndex)>,
    current_number: Res<CurrentNumber>,
) {
    for (entity, mut timer, mut color, index) in &mut query {
        if timer.0.tick(time.delta()).just_finished() {
            commands.entity(entity).remove::<FeedbackTimer>();
            // Restore color based on current active state.
            if index.0 == current_number.0 {
                color.0 = crate::constants::HIGHLIGHT_COLOR;
            } else {
                color.0 = crate::constants::TARGET_COLOR;
            }
        }
    }
}

/// System that handles user input for targets (keyboard and potentially mouse).
fn handle_target_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    sequence_state: Res<SequenceState>,
    current_number: Res<CurrentNumber>,
    mut message_writer: MessageWriter<TargetInputEvent>,
) {
    if !sequence_state.running {
        return;
    }

    let active_value = crate::constants::LABELS[current_number.0 as usize];

    // Check keys 1 to 8.
    let pressed_value = if keyboard_input.just_pressed(KeyCode::Digit1) { Some(1) }
    else if keyboard_input.just_pressed(KeyCode::Digit2) { Some(2) }
    else if keyboard_input.just_pressed(KeyCode::Digit3) { Some(3) }
    else if keyboard_input.just_pressed(KeyCode::Digit4) { Some(4) }
    else if keyboard_input.just_pressed(KeyCode::Digit5) { Some(5) }
    else if keyboard_input.just_pressed(KeyCode::Digit6) { Some(6) }
    else if keyboard_input.just_pressed(KeyCode::Digit7) { Some(7) }
    else if keyboard_input.just_pressed(KeyCode::Digit8) { Some(8) }
    else { None };

    if let Some(val) = pressed_value {
        message_writer.write(TargetInputEvent {
            correct: val == active_value,
        });
    }
}

/// System that handles the core training logic: timer ticking, sequence generation, and rhythm acceleration.
fn update_sequence_logic(
    time: Res<Time>,
    mut timer: ResMut<HighlightTimer>,
    mut current_number: ResMut<CurrentNumber>,
    mut sequence_state: ResMut<SequenceState>,
    mut rhythm_state: ResMut<RhythmState>,
) {
    if !sequence_state.running {
        return;
    }

    if timer.0.tick(time.delta()).just_finished() {
        // Handle Acceleration: Speed up every 8 steps if in Accelerate mode.
        if rhythm_state.mode == RhythmMode::Accelerate {
            rhythm_state.accelerate_counter += 1;

            if rhythm_state.accelerate_counter >= 8 {
                rhythm_state.accelerate_counter = 0;
                let mut new_duration = rhythm_state.duration - 0.1;
                if new_duration < 0.1 {
                    new_duration = 0.1; // Cap at 0.1 seconds minimum frequency.
                }

                if (new_duration - rhythm_state.duration).abs() > 0.01 {
                    rhythm_state.duration = new_duration;
                }
            }
        }

        // Determine the next target to highlight based on the current SequenceMode.
        let target_index = match sequence_state.mode {
            SequenceMode::Random => {
                let mut rng = rand::rng();
                let mut target = rng.random_range(0..=7);
                // Avoid highlighting the same number twice in a row for better training variety.
                while target == current_number.0 {
                    target = rng.random_range(0..=7);
                }
                target
            }
            SequenceMode::Ordered => {
                // Cycle values 1 to 8 sequentially.
                sequence_state.current_ordered_value =
                    (sequence_state.current_ordered_value % 8) + 1;
                // Find index of this value in the circular LABELS layout.
                crate::constants::LABELS
                    .iter()
                    .position(|&l| l == sequence_state.current_ordered_value)
                    .unwrap_or(0) as u8
            }
        };

        current_number.0 = target_index;
    }
}

/// System that syncs the visual state of the targets with the CurrentNumber resource.
fn sync_target_visuals(
    current_number: Res<CurrentNumber>,
    mut query: Query<(&NumberIndex, &mut TextColor)>,
) {
    if !current_number.is_changed() {
        return;
    }

    for (index, mut color) in &mut query {
        if index.0 == current_number.0 {
            color.0 = crate::constants::HIGHLIGHT_COLOR;
        } else {
            color.0 = crate::constants::TARGET_COLOR;
        }
    }
}

/// System that syncs the UI elements (text and slider) with changes in RhythmState.
fn sync_rhythm_ui(
    rhythm_state: Res<RhythmState>,
    mut text_query: Query<&mut Text, With<RhythmText>>,
    slider_query: Query<Entity, With<Slider>>,
    mut commands: Commands,
) {
    if !rhythm_state.is_changed() {
        return;
    }

    let duration = rhythm_state.duration;

    for mut text in &mut text_query {
        text.0 = format!("Rhythm: {:.1}s", duration);
    }
    
    for slider_entity in &slider_query {
        commands.entity(slider_entity).insert(SliderValue(duration));
    }
}

/// System that syncs the timer's duration with RhythmState.
fn sync_rhythm_timer(
    rhythm_state: Res<RhythmState>,
    mut timer: ResMut<HighlightTimer>,
) {
    if rhythm_state.is_changed() {
        timer.0.set_duration(std::time::Duration::from_secs_f32(rhythm_state.duration));
    }
}
