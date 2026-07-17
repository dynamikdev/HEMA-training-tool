use bevy::prelude::*;
use bevy_ui_widgets::SliderValue;
use std::time::Duration;

use crate::components::RhythmText;
use crate::resources::{RhythmState, HighlightTimer, RhythmMode};
use crate::ui::systems::update_rhythm_from_slider;

#[test]
fn test_update_rhythm_from_slider() {
    let mut app = App::new();

    // Set up resources.
    app.insert_resource(RhythmState {
        duration: 1.0,
        mode: RhythmMode::Constant,
        accelerate_counter: 0,
    });
    app.insert_resource(HighlightTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));

    // Spawn an entity with the RhythmText component.
    let text_entity = app.world_mut().spawn((
        Text("Rhythm: 1.0s".to_string()),
        RhythmText,
    )).id();

    // Spawn an entity with SliderValue and manually trigger the `Changed` state.
    // The value 1.55 should be rounded to 1.6 by the system.
    let slider_entity = app.world_mut().spawn(SliderValue(1.55)).id();

    app.add_systems(Update, update_rhythm_from_slider);

    // Run the system once. The initial slider spawn should trigger the `Changed` filter.
    app.update();

    // Check that RhythmState was updated and rounded correctly.
    let rhythm_state = app.world().get_resource::<RhythmState>().unwrap();
    assert_eq!(rhythm_state.duration, 1.6);

    // Check that HighlightTimer was updated.
    let highlight_timer = app.world().get_resource::<HighlightTimer>().unwrap();
    assert_eq!(highlight_timer.0.duration(), Duration::from_secs_f32(1.6));

    // Check that the text component was updated.
    let text_component = app.world().get::<Text>(text_entity).unwrap();
    assert_eq!(text_component.0, "Rhythm: 1.6s");

    // Test early return for small changes.
    // Update the slider to a value very close to the current rounded value (1.6).
    app.world_mut().entity_mut(slider_entity).insert(SliderValue(1.605));
    app.update();

    // The state should remain unchanged (1.6) because the absolute difference
    // between 1.6 and (1.605 * 10).round() / 10.0 => 1.6 is 0.0, which is < 0.01.
    let rhythm_state = app.world().get_resource::<RhythmState>().unwrap();
    assert_eq!(rhythm_state.duration, 1.6);
}
