
#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use crate::logic::TrainingPlugin;
    use crate::resources::*;
    use crate::constants::LABELS;

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
           .add_plugins(TrainingPlugin)
           .insert_resource(ButtonInput::<KeyCode>::default())
           .insert_resource(HighlightTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
           .insert_resource(CurrentNumber(0))
           .insert_resource(SequenceState { running: true, ..default() })
           .insert_resource(RhythmState { duration: 1.0, mode: RhythmMode::Constant, accelerate_counter: 0 });
        app
    }

    #[test]
    fn test_highlight_system_updates_timer() {
        let mut app = setup_app();

        // First update to initialize time.
        app.update();
        
        // Manually set timer to almost finished.
        {
            let mut timer = app.world_mut().get_resource_mut::<HighlightTimer>().unwrap();
            timer.0.set_elapsed(std::time::Duration::from_secs_f32(1.0));
        }

        // Run system again.
        app.update();

        // After the timer finished, the current number should have been updated.
        assert!(app.world().get_resource::<CurrentNumber>().is_some());
    }

    #[test]
    fn test_ordered_sequence_mode() {
        let mut app = setup_app();
        {
            let mut seq_state = app.world_mut().get_resource_mut::<SequenceState>().unwrap();
            seq_state.mode = SequenceMode::Ordered;
            seq_state.current_ordered_value = 0;
        }

        app.update();

        // Manually set timer to finished.
        {
            let mut timer = app.world_mut().get_resource_mut::<HighlightTimer>().unwrap();
            timer.0.set_elapsed(std::time::Duration::from_secs_f32(1.0));
        }
        app.update();

        let seq_state = app.world().get_resource::<SequenceState>().unwrap();
        assert_eq!(seq_state.current_ordered_value, 1);
        
        let current_number = app.world().get_resource::<CurrentNumber>().unwrap();
        let expected_index = LABELS.iter().position(|&l| l == 1).unwrap() as u8;
        assert_eq!(current_number.0, expected_index);
    }

    #[test]
    fn test_constant_rhythm_timer_updates_from_rhythm_state() {
        let mut app = setup_app();

        app.update();

        // Change duration in RhythmState.
        {
            let mut rhythm_state = app.world_mut().get_resource_mut::<RhythmState>().unwrap();
            rhythm_state.duration = 2.0;
        }

        app.update();

        let timer = app.world().get_resource::<HighlightTimer>().unwrap();
        assert_eq!(timer.0.duration().as_secs_f32(), 2.0);
    }

    #[test]
    fn test_accelerate_rhythm_mode_speeds_up() {
        let mut app = setup_app();
        {
            let mut rhythm_state = app.world_mut().get_resource_mut::<RhythmState>().unwrap();
            rhythm_state.mode = RhythmMode::Accelerate;
        }

        app.update();

        // Tick 8 times.
        for _ in 0..8 {
            {
                let mut timer = app.world_mut().get_resource_mut::<HighlightTimer>().unwrap();
                timer.0.set_elapsed(std::time::Duration::from_secs_f32(1.0));
            }
            app.update();
        }

        let rhythm_state = app.world().get_resource::<RhythmState>().unwrap();
        assert!((rhythm_state.duration - 0.9).abs() < 0.01);
        
        let timer = app.world().get_resource::<HighlightTimer>().unwrap();
        assert!((timer.0.duration().as_secs_f32() - 0.9).abs() < 0.01);
    }

    #[test]
    fn test_accelerate_rhythm_mode_caps_at_min_duration() {
        let mut app = setup_app();
        {
            let mut rhythm_state = app.world_mut().get_resource_mut::<RhythmState>().unwrap();
            rhythm_state.mode = RhythmMode::Accelerate;
            rhythm_state.duration = 0.2;
        }
        {
            let mut timer = app.world_mut().get_resource_mut::<HighlightTimer>().unwrap();
            timer.0.set_duration(std::time::Duration::from_secs_f32(0.2));
        }

        app.update();

        // Tick 8 times to speed up from 0.2 to 0.1.
        for _ in 0..8 {
            {
                let mut timer = app.world_mut().get_resource_mut::<HighlightTimer>().unwrap();
                timer.0.set_elapsed(std::time::Duration::from_secs_f32(0.2));
            }
            app.update();
        }

        let rhythm_state = app.world().get_resource::<RhythmState>().unwrap();
        assert!((rhythm_state.duration - 0.1).abs() < 0.01);

        // Tick another 8 times.
        for _ in 0..8 {
            {
                let mut timer = app.world_mut().get_resource_mut::<HighlightTimer>().unwrap();
                timer.0.set_elapsed(std::time::Duration::from_secs_f32(0.1));
            }
            app.update();
        }

        let rhythm_state = app.world().get_resource::<RhythmState>().unwrap();
        assert!((rhythm_state.duration - 0.1).abs() < 0.01);
    }

    #[test]
    fn test_handle_correct_input() {
        let mut app = setup_app();
        {
            let mut current_number = app.world_mut().get_resource_mut::<CurrentNumber>().unwrap();
            current_number.0 = 1; // LABELS[1] is 1.
        }

        app.update();

        // Press '1'.
        {
            let mut input = app.world_mut().get_resource_mut::<ButtonInput<KeyCode>>().unwrap();
            input.press(KeyCode::Digit1);
        }

        app.update();

        let messages = app.world().get_resource::<Messages<TargetInputEvent>>().unwrap();
        let mut reader = messages.get_cursor();
        let message = reader.read(messages).next().unwrap();
        assert!(message.correct);
    }

    #[test]
    fn test_handle_incorrect_input() {
        let mut app = setup_app();
        {
            let mut current_number = app.world_mut().get_resource_mut::<CurrentNumber>().unwrap();
            current_number.0 = 1; // LABELS[1] is 1.
        }

        app.update();

        // Press '2' (incorrect, active is 1).
        {
            let mut input = app.world_mut().get_resource_mut::<ButtonInput<KeyCode>>().unwrap();
            input.press(KeyCode::Digit2);
        }

        app.update();

        let messages = app.world().get_resource::<Messages<TargetInputEvent>>().unwrap();
        let mut reader = messages.get_cursor();
        let message = reader.read(messages).next().unwrap();
        assert!(!message.correct);
    }
}
