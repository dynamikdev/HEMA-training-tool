//! Tests for the UI systems logic.

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use crate::components::SequenceControlButton;
    use crate::constants::PRIMARY_EMISSIVE;
    use crate::resources::{RhythmMode, RhythmState, SequenceState};
    use crate::ui::systems::sequence_control_button_system;

    #[test]
    fn test_sequence_control_button_system() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Initialize resources
        app.insert_resource(SequenceState::default());
        app.insert_resource(RhythmState {
            duration: 1.0,
            mode: RhythmMode::Constant,
            accelerate_counter: 5, // Set to some value to test if it gets reset
        });

        // Add the system
        app.add_systems(Update, sequence_control_button_system);

        // Spawn a Text entity to be the child of the button
        let text_entity = app.world_mut().spawn(Text::new("Launch Sequence")).id();

        // Spawn the button entity
        let button_entity = app.world_mut().spawn((
            SequenceControlButton,
            Interaction::None,
            BackgroundColor(Color::NONE),
        )).add_child(text_entity).id();

        app.update();

        // 1. Test Hovered State
        *app.world_mut().get_mut::<Interaction>(button_entity).unwrap() = Interaction::Hovered;
        app.update();

        let bg_color = app.world().get::<BackgroundColor>(button_entity).unwrap().0;
        assert_eq!(bg_color, Color::srgba(0.886, 0.886, 0.886, 0.1));

        // 2. Test Pressed State (Start Sequence)
        *app.world_mut().get_mut::<Interaction>(button_entity).unwrap() = Interaction::Pressed;
        app.update();

        let seq_state = app.world().resource::<SequenceState>();
        assert!(seq_state.running);

        let bg_color = app.world().get::<BackgroundColor>(button_entity).unwrap().0;
        assert_eq!(bg_color, PRIMARY_EMISSIVE);

        let text = app.world().get::<Text>(text_entity).unwrap();
        assert_eq!(text.0, "Stop Sequence");

        // 3. Test None State after starting (Interaction::None behavior to be fixed)
        *app.world_mut().get_mut::<Interaction>(button_entity).unwrap() = Interaction::None;
        app.update();

        let bg_color = app.world().get::<BackgroundColor>(button_entity).unwrap().0;
        assert_eq!(bg_color, PRIMARY_EMISSIVE);

        // 4. Test Pressed State (Stop Sequence)
        *app.world_mut().get_mut::<Interaction>(button_entity).unwrap() = Interaction::Pressed;
        app.update();

        let seq_state = app.world().resource::<SequenceState>();
        assert!(!seq_state.running);

        let rhythm_state = app.world().resource::<RhythmState>();
        assert_eq!(rhythm_state.accelerate_counter, 0);

        let bg_color = app.world().get::<BackgroundColor>(button_entity).unwrap().0;
        assert_eq!(bg_color, Color::NONE);

        let text = app.world().get::<Text>(text_entity).unwrap();
        assert_eq!(text.0, "Launch Sequence");

        // 5. Test None State after stopping
        *app.world_mut().get_mut::<Interaction>(button_entity).unwrap() = Interaction::None;
        app.update();

        let bg_color = app.world().get::<BackgroundColor>(button_entity).unwrap().0;
        assert_eq!(bg_color, Color::NONE);
    }
}
