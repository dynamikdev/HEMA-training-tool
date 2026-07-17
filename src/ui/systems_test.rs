//! Tests for the UI interaction systems.

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use crate::ui::systems::mode_toggle_system;
    use crate::resources::{SequenceState, SequenceMode};
    use crate::components::SequenceModeButton;

    #[test]
    fn test_mode_toggle_system_random_to_ordered() {
        let mut app = App::new();

        let mut seq_state = SequenceState::default();
        seq_state.mode = SequenceMode::Random;
        seq_state.current_ordered_value = 5;
        app.insert_resource(seq_state);

        app.add_systems(Update, mode_toggle_system);

        let entity = app.world_mut().spawn((
            SequenceModeButton,
            Interaction::Pressed,
            BackgroundColor(Color::NONE),
        )).id();
        let text_entity = app.world_mut().spawn(Text::new("Mode: Random")).id();
        app.world_mut().entity_mut(entity).add_child(text_entity);

        app.update();

        let state = app.world().resource::<SequenceState>();
        assert_eq!(state.mode, SequenceMode::Ordered, "Mode should change to Ordered.");
        assert_eq!(state.current_ordered_value, 0, "Ordered value should reset to 0.");

        let text = app.world().get::<Text>(text_entity).unwrap();
        assert_eq!(text.0, "Mode: Ordered", "Text should update to Mode: Ordered.");
    }

    #[test]
    fn test_mode_toggle_system_ordered_to_random() {
        let mut app = App::new();

        let mut seq_state = SequenceState::default();
        seq_state.mode = SequenceMode::Ordered;
        seq_state.current_ordered_value = 5;
        app.insert_resource(seq_state);

        app.add_systems(Update, mode_toggle_system);

        let entity = app.world_mut().spawn((
            SequenceModeButton,
            Interaction::Pressed,
            BackgroundColor(Color::NONE),
        )).id();
        let text_entity = app.world_mut().spawn(Text::new("Mode: Ordered")).id();
        app.world_mut().entity_mut(entity).add_child(text_entity);

        app.update();

        let state = app.world().resource::<SequenceState>();
        assert_eq!(state.mode, SequenceMode::Random, "Mode should change to Random.");
        assert_eq!(state.current_ordered_value, 0, "Ordered value should reset to 0.");

        let text = app.world().get::<Text>(text_entity).unwrap();
        assert_eq!(text.0, "Mode: Random", "Text should update to Mode: Random.");
    }

    #[test]
    fn test_mode_toggle_system_hovered() {
        let mut app = App::new();
        app.insert_resource(SequenceState::default());
        app.add_systems(Update, mode_toggle_system);

        let entity = app.world_mut().spawn((
            SequenceModeButton,
            Interaction::Hovered,
            BackgroundColor(Color::NONE),
        )).id();
        let text_entity = app.world_mut().spawn(Text::new("Mode: Random")).id();
        app.world_mut().entity_mut(entity).add_child(text_entity);

        app.update();

        let bg_color = app.world().get::<BackgroundColor>(entity).unwrap();
        assert_eq!(bg_color.0, Color::srgba(0.886, 0.886, 0.886, 0.1), "Background color should be subtle gray on hover.");
    }

    #[test]
    fn test_mode_toggle_system_none() {
        let mut app = App::new();
        app.insert_resource(SequenceState::default());
        app.add_systems(Update, mode_toggle_system);

        let entity = app.world_mut().spawn((
            SequenceModeButton,
            Interaction::None,
            BackgroundColor(Color::srgba(1.0, 0.0, 0.0, 1.0)),
        )).id();
        let text_entity = app.world_mut().spawn(Text::new("Mode: Random")).id();
        app.world_mut().entity_mut(entity).add_child(text_entity);

        app.update();

        let bg_color = app.world().get::<BackgroundColor>(entity).unwrap();
        assert_eq!(bg_color.0, Color::NONE, "Background color should reset to NONE.");
    }
}
