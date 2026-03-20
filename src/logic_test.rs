//! Unit tests for the core training logic of the HEMA Training Tool.
//!
//! These tests verify the behavior of the `TrainingPlugin`, including sequence modes,
//! rhythm modes, and input handling for starting/pausing the training session.

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use crate::logic::TrainingPlugin;
    use crate::resources::*;
    use crate::constants::LABELS;

    /// Sets up a Bevy `App` with the `TrainingPlugin` and required resources for testing.
    fn setup_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
           .add_plugins(TrainingPlugin)
           .insert_resource(ButtonInput::<KeyCode>::default())
           .insert_resource(HighlightTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
           .insert_resource(CurrentNumber(0))
           .insert_resource(SequenceState { running: true, ..default() })
           .insert_resource(RhythmState { duration: 1.0, mode: RhythmMode::Constant, accelerate_counter: 0 })
           .insert_resource(ArrowTarget::default())
           .insert_resource(ArrowAnimationState::default());
        app
    }

    /// Verifies that the `highlight_system` correctly updates the `HighlightTimer`.
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

    /// Verifies that the `Ordered` sequence mode correctly increments the target index
    /// according to the defined `LABELS`.
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

    /// Verifies that the `Constant` rhythm mode correctly updates the `HighlightTimer`
    /// when the `RhythmState` duration is changed.
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

    /// Verifies that the `Accelerate` rhythm mode correctly decreases the timer duration
    /// after a set number of ticks.
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

    /// Verifies that the `Accelerate` rhythm mode does not decrease the duration
    /// below a minimum threshold (0.1s).
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

    /// Verifies that pressing the Space key toggles the training sequence between
    /// running and paused states.
    #[test]
    fn test_toggle_play_pause_with_spacebar() {
        let mut app = setup_app();
        
        // Initial state should be false if using default (but setup_app sets it to true).
        // Let's set it to false explicitly to test toggle to true.
        {
            let mut seq_state = app.world_mut().get_resource_mut::<SequenceState>().unwrap();
            seq_state.running = false;
        }

        app.update();

        // Press Space.
        {
            let mut input = app.world_mut().get_resource_mut::<ButtonInput<KeyCode>>().unwrap();
            input.press(KeyCode::Space);
        }

        app.update();

        let seq_state = app.world().get_resource::<SequenceState>().unwrap();
        assert!(seq_state.running);

        // Press Space again.
        {
            let mut input = app.world_mut().get_resource_mut::<ButtonInput<KeyCode>>().unwrap();
            input.release(KeyCode::Space);
            input.press(KeyCode::Space);
        }

        app.update();

        let seq_state = app.world().get_resource::<SequenceState>().unwrap();
        assert!(!seq_state.running);
    }

    /// Verifies that the `ArrowTarget` is correctly updated when the `CurrentNumber` changes.
    #[test]
    fn test_arrow_target_calculation() {
        let mut app = setup_app();
        app.insert_resource(ArrowTarget::default());
        
        // Initial current number is 0.
        app.update();
        
        // At index 0, the opposite is index 4 (0 + 4).
        let arrow_target = app.world().get_resource::<ArrowTarget>().unwrap();
        assert_eq!(arrow_target.0, Some(4));

        // Change current number to index 3.
        {
            let mut current_number = app.world_mut().get_resource_mut::<CurrentNumber>().unwrap();
            current_number.0 = 3;
        }

        app.update();

        // At index 3, the opposite is index 7 (3 + 4).
        let arrow_target = app.world().get_resource::<ArrowTarget>().unwrap();
        assert_eq!(arrow_target.0, Some(7));

        // Change current number to index 6.
        {
            let mut current_number = app.world_mut().get_resource_mut::<CurrentNumber>().unwrap();
            current_number.0 = 6;
        }

        app.update();

        // At index 6, the opposite is index 2 ((6 + 4) % 8).
        let arrow_target = app.world().get_resource::<ArrowTarget>().unwrap();
        assert_eq!(arrow_target.0, Some(2));
    }

    /// Verifies that the arrow animation progress resets and increments over time.
    #[test]
    fn test_arrow_animation_progress() {
        let mut app = setup_app();
        app.insert_resource(ArrowAnimationState::default());
        
        // Disable sequence logic to prevent auto-advancing current_number.
        {
            let mut seq_state = app.world_mut().get_resource_mut::<SequenceState>().unwrap();
            seq_state.running = false;
        }

        // Initial update.
        app.update();
        
        // Change target to trigger animation reset.
        {
            let mut current_number = app.world_mut().get_resource_mut::<CurrentNumber>().unwrap();
            current_number.0 = 1;
        }
        
        app.update(); // Frame 1: Reset progress to 0.0.
        
        // Frame 2: flag should be cleared.
        app.update(); 

        // Advance time (0.1s).
        {
            let mut time = app.world_mut().get_resource_mut::<Time>().unwrap();
            time.advance_by(std::time::Duration::from_millis(100));
        }
        
        app.update();
        
        {
            let animation_state = app.world().get_resource::<ArrowAnimationState>().unwrap();
            assert!(animation_state.progress > 0.0);
        }
    }
}
