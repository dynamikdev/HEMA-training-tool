//! Core training simulation logic for target highlighting and rhythm control.

use bevy::prelude::*;
use bevy_ui_widgets::{SetSliderValue, Slider, SliderValueChange};
use rand::Rng;

use crate::components::*;
use crate::constants::LABELS;
use crate::resources::*;

/// Plugin that handles the core training logic, such as the highlighting system.
pub struct TrainingPlugin;

impl Plugin for TrainingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, highlight_system);
    }
}

/// System that handles the periodic highlighting of target numbers based on the selected mode and rhythm.
fn highlight_system(
    time: Res<Time>,
    mut timer: ResMut<HighlightTimer>,
    mut highlighted_number: ResMut<CurrentNumber>,
    mut query: Query<(&NumberIndex, &mut TextColor)>,
    mut sequence_state: ResMut<SequenceState>,
    mut rhythm_state: ResMut<RhythmState>,
    mut text_query: Query<&mut Text, With<RhythmText>>,
    slider_query: Query<Entity, With<Slider>>,
    mut commands: Commands,
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

                if new_duration != rhythm_state.duration {
                    rhythm_state.duration = new_duration;
                    timer
                        .0
                        .set_duration(std::time::Duration::from_secs_f32(new_duration));

                    // Sync UI text display and the slider widget position.
                    for mut text in &mut text_query {
                        text.0 = format!("Rhythm: {:.1}s", new_duration);
                    }
                    for slider_entity in &slider_query {
                        commands.trigger(SetSliderValue {
                            entity: slider_entity,
                            change: SliderValueChange::Absolute(new_duration),
                        });
                    }
                }
            }
        }

        // Determine the next target to highlight based on the current SequenceMode.
        let target_index = match sequence_state.mode {
            SequenceMode::Random => {
                let mut rng = rand::rng();
                let mut target = rng.random_range(0..=7);
                // Avoid highlighting the same number twice in a row for better training variety.
                while target == highlighted_number.0 {
                    target = rng.random_range(0..=7);
                }
                target
            }
            SequenceMode::Ordered => {
                // Cycle values 1 to 8 sequentially.
                sequence_state.current_ordered_value =
                    (sequence_state.current_ordered_value % 8) + 1;
                // Find index of this value in the circular LABELS layout.
                LABELS
                    .iter()
                    .position(|&l| l == sequence_state.current_ordered_value)
                    .unwrap_or(0) as u8
            }
        };

        highlighted_number.0 = target_index;

        // Apply visual feedback by changing the color of the target text.
        for (index, mut color) in &mut query {
            if index.0 == target_index {
                // High-intensity red for the active target.
                color.0 = Color::srgb(5.0, 0.0, 0.0);
            } else {
                color.0 = Color::WHITE;
            }
        }
    }
}
