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
        app.add_systems(Update, (
            update_sequence_logic,
            handle_session_controls,
            sync_rhythm_timer,
            sync_rhythm_ui
        ).chain());
    }
}

/// System that handles overall session controls like Play/Pause.
fn handle_session_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut sequence_state: ResMut<SequenceState>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        sequence_state.running = !sequence_state.running;
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
