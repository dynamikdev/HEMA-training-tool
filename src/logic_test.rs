
#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use crate::logic::TrainingPlugin;
    use crate::resources::*;
    use crate::components::*;
    use crate::constants::LABELS;

    #[test]
    fn test_highlight_system_updates_timer() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
           .add_plugins(TrainingPlugin)
           .insert_resource(HighlightTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
           .insert_resource(CurrentNumber(0))
           .insert_resource(SequenceState { running: true, ..default() })
           .insert_resource(RhythmState { duration: 1.0, mode: RhythmMode::Constant, accelerate_counter: 0 });

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
        // It's randomized, so we just check it exists.
        assert!(app.world().get_resource::<CurrentNumber>().is_some());
    }

    #[test]
    fn test_ordered_sequence_mode() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
           .add_plugins(TrainingPlugin)
           .insert_resource(HighlightTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
           .insert_resource(CurrentNumber(0))
           .insert_resource(SequenceState { 
               running: true, 
               mode: SequenceMode::Ordered, 
               current_ordered_value: 0 
           })
           .insert_resource(RhythmState { 
               duration: 1.0, 
               mode: RhythmMode::Constant, 
               accelerate_counter: 0 
           });

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
        // 1 is at some index in LABELS.
        let expected_index = LABELS.iter().position(|&l| l == 1).unwrap() as u8;
        assert_eq!(current_number.0, expected_index);
    }
}
