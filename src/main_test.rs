//! Integration tests for the application initialization.
//!
//! These tests verify that the `initialize_app` function correctly configures the
//! Bevy `App` with all necessary resources and plugins.

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use crate::initialize_app;
    use crate::resources::*;

    /// Verifies that `initialize_app` correctly adds the required training resources
    /// to the Bevy `App`.
    #[test]
    fn test_initialize_app_adds_resources() {
        let mut app = App::new();
        initialize_app(&mut app);

        // Verify that initial resources were correctly inserted.
        assert!(app.world().get_resource::<HighlightTimer>().is_some());
        assert!(app.world().get_resource::<CurrentNumber>().is_some());
        assert!(app.world().get_resource::<SequenceState>().is_some());
        assert!(app.world().get_resource::<RhythmState>().is_some());
        assert!(app.world().get_resource::<Typography>().is_some());
        assert!(app.world().get_resource::<CurriculumState>().is_some());
    }
}
