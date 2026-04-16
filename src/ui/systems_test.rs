//! Tests for the UI systems module.
#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use crate::components::RhythmModeButton;
    use crate::resources::{RhythmState, RhythmMode};
    use crate::ui::systems::rhythm_mode_toggle_system;

    fn setup_app() -> App {
        let mut app = App::new();
        // Since bevy::core doesn't have it and it's not in prelude let's just use MinimalPlugins to set up App schedule.
        app.add_plugins(MinimalPlugins);
        app
    }

    #[test]
    fn test_rhythm_mode_toggle_system_pressed_constant_to_accelerate() {
        let mut app = setup_app();

        app.insert_resource(RhythmState {
            mode: RhythmMode::Constant,
            duration: 1.0,
            accelerate_counter: 5,
        });

        app.add_systems(Update, rhythm_mode_toggle_system);

        let text_entity = app.world_mut().spawn(Text::new("Rhythm: Constant")).id();
        let _button_entity = app.world_mut().spawn((
            RhythmModeButton,
            Interaction::Pressed,
            BackgroundColor(Color::NONE),
        )).add_child(text_entity).id();

        app.update();

        let state = app.world().resource::<RhythmState>();
        assert_eq!(state.mode, RhythmMode::Accelerate);
        assert_eq!(state.accelerate_counter, 0);

        let text = app.world().get::<Text>(text_entity).unwrap();
        assert_eq!(text.0, "Rhythm: Accelerate");
    }

    #[test]
    fn test_rhythm_mode_toggle_system_pressed_accelerate_to_constant() {
        let mut app = setup_app();

        app.insert_resource(RhythmState {
            mode: RhythmMode::Accelerate,
            duration: 1.0,
            accelerate_counter: 8,
        });

        app.add_systems(Update, rhythm_mode_toggle_system);

        let text_entity = app.world_mut().spawn(Text::new("Rhythm: Accelerate")).id();
        let _button_entity = app.world_mut().spawn((
            RhythmModeButton,
            Interaction::Pressed,
            BackgroundColor(Color::NONE),
        )).add_child(text_entity).id();

        app.update();

        let state = app.world().resource::<RhythmState>();
        assert_eq!(state.mode, RhythmMode::Constant);
        assert_eq!(state.accelerate_counter, 0);

        let text = app.world().get::<Text>(text_entity).unwrap();
        assert_eq!(text.0, "Rhythm: Constant");
    }

    #[test]
    fn test_rhythm_mode_toggle_system_hovered() {
        let mut app = setup_app();

        app.insert_resource(RhythmState {
            mode: RhythmMode::Constant,
            duration: 1.0,
            accelerate_counter: 0,
        });

        app.add_systems(Update, rhythm_mode_toggle_system);

        let text_entity = app.world_mut().spawn(Text::new("Rhythm: Constant")).id();
        let button_entity = app.world_mut().spawn((
            RhythmModeButton,
            Interaction::Hovered,
            BackgroundColor(Color::NONE),
        )).add_child(text_entity).id();

        app.update();

        let state = app.world().resource::<RhythmState>();
        assert_eq!(state.mode, RhythmMode::Constant); // Should not change

        let background = app.world().get::<BackgroundColor>(button_entity).unwrap();
        assert_eq!(background.0, Color::srgba(0.886, 0.886, 0.886, 0.1));
    }

    #[test]
    fn test_rhythm_mode_toggle_system_none() {
        let mut app = setup_app();

        app.insert_resource(RhythmState {
            mode: RhythmMode::Constant,
            duration: 1.0,
            accelerate_counter: 0,
        });

        app.add_systems(Update, rhythm_mode_toggle_system);

        let text_entity = app.world_mut().spawn(Text::new("Rhythm: Constant")).id();
        let button_entity = app.world_mut().spawn((
            RhythmModeButton,
            Interaction::None,
            BackgroundColor(Color::srgba(0.886, 0.886, 0.886, 0.1)),
        )).add_child(text_entity).id();

        app.update();

        let state = app.world().resource::<RhythmState>();
        assert_eq!(state.mode, RhythmMode::Constant); // Should not change

        let background = app.world().get::<BackgroundColor>(button_entity).unwrap();
        assert_eq!(background.0, Color::NONE);
    }
}
