use bevy::prelude::*;
use bevy_ui_widgets::{SetSliderValue, Slider, SliderValueChange};
use rand::Rng;

use crate::components::*;
use crate::constants::LABELS;
use crate::resources::*;

pub struct TrainingPlugin;

impl Plugin for TrainingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, highlight_system);
    }
}

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
        // Handle Acceleration
        if rhythm_state.mode == RhythmMode::Accelerate {
            rhythm_state.accelerate_counter += 1;

            if rhythm_state.accelerate_counter >= 8 {
                rhythm_state.accelerate_counter = 0;
                let mut new_duration = rhythm_state.duration - 0.1;
                if new_duration < 0.1 {
                    new_duration = 0.1; // Cap at 0.1 seconds minimum
                }

                if new_duration != rhythm_state.duration {
                    rhythm_state.duration = new_duration;
                    timer
                        .0
                        .set_duration(std::time::Duration::from_secs_f32(new_duration));

                    // Sync UI text and slider
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
                sequence_state.current_ordered_value =
                    (sequence_state.current_ordered_value % 8) + 1;
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
