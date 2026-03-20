//! Core training simulation logic, including target highlighting and rhythm control.
//!
//! This module contains the main Bevy plugin and systems that drive the training
//! experience by managing timers, generating sequences, and handling session controls.

use bevy::prelude::*;
use bevy_ui_widgets::{Slider, SliderValue};
use rand::Rng;

use crate::components::*;
use crate::resources::*;

/// A Bevy plugin that encapsulates all training simulation logic.
///
/// This plugin registers the core systems responsible for the training loop
/// and ensures they run in a predictable sequence.
pub struct TrainingPlugin;

impl Plugin for TrainingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_sequence_logic,
            handle_session_controls,
            sync_rhythm_timer,
            sync_arrow_target,
            sync_rhythm_ui
        ).chain());
    }
}

/// Toggles the training session on or off in response to user input.
///
/// Currently, this system listens for the Space key to play or pause the
/// sequence, allowing the user to start or halt their practice session.
fn handle_session_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut sequence_state: ResMut<SequenceState>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        sequence_state.running = !sequence_state.running;
    }
}

/// The core system that advances the training sequence.
///
/// This system performs the following duties when the session is active:
/// 1. Ticks the [`HighlightTimer`].
/// 2. If the timer finishes:
///    - Handles rhythm acceleration in [`RhythmMode::Accelerate`].
///    - Generates the next target index based on the active [`SequenceMode`].
///    - Updates the global [`CurrentNumber`] resource to trigger UI updates.
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
        // Accelerate: Decrease duration every 8 steps.
        if rhythm_state.mode == RhythmMode::Accelerate {
            rhythm_state.accelerate_counter += 1;

            if rhythm_state.accelerate_counter >= 8 {
                rhythm_state.accelerate_counter = 0;
                let mut new_duration = rhythm_state.duration - 0.1;
                // Floor duration at 0.1s to prevent the trainer from becoming impossible.
                if new_duration < 0.1 {
                    new_duration = 0.1;
                }

                // Check for actual change to minimize unnecessary updates.
                if (new_duration - rhythm_state.duration).abs() > 0.01 {
                    rhythm_state.duration = new_duration;
                }
            }
        }

        // Generate the next target index.
        let target_index = match sequence_state.mode {
            SequenceMode::Random => {
                let mut rng = rand::rng();
                let mut target = rng.random_range(0..=7);
                // Ensure a different target is picked to maintain user engagement and focus.
                while target == current_number.0 {
                    target = rng.random_range(0..=7);
                }
                target
            }
            SequenceMode::Ordered => {
                // Advance the numeric value (1 through 8).
                sequence_state.current_ordered_value =
                    (sequence_state.current_ordered_value % 8) + 1;
                
                // Map the human-readable label to its visual position index.
                crate::constants::LABELS
                    .iter()
                    .position(|&l| l == sequence_state.current_ordered_value)
                    .unwrap_or(0) as u8
            }
        };

        current_number.0 = target_index;
    }
}

/// Keeps UI elements synchronized with the internal [`RhythmState`].
///
/// Updates the numeric display and the slider widget whenever the rhythm
/// duration is modified (either by user input or auto-acceleration).
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

    // Update the textual feedback for the user.
    for mut text in &mut text_query {
        text.0 = format!("Rhythm: {:.1}s", duration);
    }
    
    // Update the slider widget to reflect the new internal state.
    for slider_entity in &slider_query {
        commands.entity(slider_entity).insert(SliderValue(duration));
    }
}

/// Updates the hardware-backed timer whenever the desired rhythm duration changes.
///
/// This ensures that the simulation's tick frequency remains in sync with the
/// [`RhythmState`]'s duration parameter.
fn sync_rhythm_timer(
    rhythm_state: Res<RhythmState>,
    mut timer: ResMut<HighlightTimer>,
) {
    if rhythm_state.is_changed() {
        timer.0.set_duration(std::time::Duration::from_secs_f32(rhythm_state.duration));
    }
}

/// Recalculates the diametrically opposite target whenever the active target changes.
///
/// This system updates the [`ArrowTarget`] resource, which is then used by
/// visual systems to position the guide arrow.
fn sync_arrow_target(
    current_number: Res<CurrentNumber>,
    mut arrow_target: ResMut<ArrowTarget>,
) {
    if current_number.is_changed() {
        // In a circular layout of 8 targets, the target diametrically opposite
        // to index 'i' is always '(i + 4) % 8'.
        arrow_target.0 = Some((current_number.0 + 4) % 8);
    }
}
