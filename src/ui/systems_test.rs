#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use crate::components::SequenceControlButton;
    use crate::constants::{PRIMARY_EMISSIVE, GHOST_BORDER};
    use crate::resources::{RhythmMode, RhythmState, SequenceMode, SequenceState};
    use crate::ui::systems::sequence_control_button_system;

    /// Helper function to create a minimal Bevy app for testing the sequence_control_button_system
    fn setup_test_app() -> App {
        let mut app = App::new();
        // Add minimal plugins required to avoid 'TaskPool has not been initialized yet' panics
        app.add_plugins(TaskPoolPlugin::default());

        app.insert_resource(SequenceState {
            running: false,
            mode: SequenceMode::Random,
            current_ordered_value: 0,
        });

        app.insert_resource(RhythmState {
            duration: 1.0,
            mode: RhythmMode::Constant,
            accelerate_counter: 0,
        });

        app.add_systems(Update, sequence_control_button_system);

        app
    }

    #[test]
    fn test_sequence_control_button_pressed_starts_sequence() {
        let mut app = setup_test_app();

        // Ensure sequence state starts as not running
        assert!(!app.world().resource::<SequenceState>().running);

        let text_entity = app.world_mut().spawn(Text::new("Launch Sequence")).id();
        let button_entity = app.world_mut().spawn((
            SequenceControlButton,
            Interaction::Pressed,
            BackgroundColor(Color::NONE),
        )).add_child(text_entity).id();

        app.update();

        // Sequence should now be running
        let sequence_state = app.world().resource::<SequenceState>();
        assert!(sequence_state.running);

        // Text and color should be updated
        let text = app.world().get::<Text>(text_entity).unwrap();
        assert_eq!(text.0, "Stop Sequence");

        let background_color = app.world().get::<BackgroundColor>(button_entity).unwrap();
        assert_eq!(background_color.0, PRIMARY_EMISSIVE);
    }

    #[test]
    fn test_sequence_control_button_pressed_stops_sequence() {
        let mut app = setup_test_app();

        // Set state as if it was already running
        app.world_mut().resource_mut::<SequenceState>().running = true;
        app.world_mut().resource_mut::<RhythmState>().accelerate_counter = 5;

        let text_entity = app.world_mut().spawn(Text::new("Stop Sequence")).id();
        let button_entity = app.world_mut().spawn((
            SequenceControlButton,
            Interaction::Pressed,
            BackgroundColor(PRIMARY_EMISSIVE),
        )).add_child(text_entity).id();

        app.update();

        // Sequence should now be stopped
        let sequence_state = app.world().resource::<SequenceState>();
        assert!(!sequence_state.running);

        // Rhythm state should have its counter reset
        let rhythm_state = app.world().resource::<RhythmState>();
        assert_eq!(rhythm_state.accelerate_counter, 0);

        // Text and color should be updated
        let text = app.world().get::<Text>(text_entity).unwrap();
        assert_eq!(text.0, "Launch Sequence");

        let background_color = app.world().get::<BackgroundColor>(button_entity).unwrap();
        assert_eq!(background_color.0, Color::NONE);
    }

    #[test]
    fn test_sequence_control_button_hovered() {
        let mut app = setup_test_app();

        let text_entity = app.world_mut().spawn(Text::new("Launch Sequence")).id();
        let button_entity = app.world_mut().spawn((
            SequenceControlButton,
            Interaction::Hovered,
            BackgroundColor(Color::NONE),
        )).add_child(text_entity).id();

        app.update();

        let background_color = app.world().get::<BackgroundColor>(button_entity).unwrap();
        assert_eq!(background_color.0, GHOST_BORDER);
    }

    #[test]
    fn test_sequence_control_button_none_restores_color() {
        let mut app = setup_test_app();

        let text_entity = app.world_mut().spawn(Text::new("Launch Sequence")).id();
        let button_entity = app.world_mut().spawn((
            SequenceControlButton,
            Interaction::None, // Emulate changed to None
            BackgroundColor(GHOST_BORDER), // It was hovered before
        )).add_child(text_entity).id();

        // Ensure button component is marked as changed by artificially sending an update
        app.update();
        app.world_mut().get_mut::<Interaction>(button_entity).unwrap().set_changed();
        app.update();

        // Since running is false, it should restore to Color::NONE
        let background_color = app.world().get::<BackgroundColor>(button_entity).unwrap();
        assert_eq!(background_color.0, Color::NONE);

        // Now test when running is true
        app.world_mut().resource_mut::<SequenceState>().running = true;

        // Emulate hovering -> None
        app.world_mut().get_mut::<BackgroundColor>(button_entity).unwrap().0 = GHOST_BORDER;
        app.world_mut().get_mut::<Interaction>(button_entity).unwrap().set_changed();

        app.update();

        let background_color = app.world().get::<BackgroundColor>(button_entity).unwrap();
        assert_eq!(background_color.0, PRIMARY_EMISSIVE);
    }
}
