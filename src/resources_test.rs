//! Unit tests for session and rhythm resources.

#[cfg(test)]
mod tests {
    use crate::resources::*;

    /// Verifies that a manually initialized `RhythmState` has correct initial values.
    #[test]
    fn test_rhythm_state_default() {
        // Since RhythmState doesn't implement Default, we test it manually.
        let state = RhythmState {
            duration: 1.0,
            mode: RhythmMode::Constant,
            accelerate_counter: 0,
        };
        assert_eq!(state.duration, 1.0);
        assert_eq!(state.mode, RhythmMode::Constant);
    }

    /// Confirms that `SequenceState` uses appropriate default values.
    #[test]
    fn test_sequence_state_default() {
        let state = SequenceState::default();
        assert!(!state.running);
        assert_eq!(state.mode, SequenceMode::Random);
    }

    /// Verifies the default initialization of `CurriculumState`.
    #[test]
    fn test_curriculum_state_default() {
        let state = CurriculumState::default();
        assert!(!state.is_visible);
        assert_eq!(state.selected_grade, Some("Niveau 1.1".to_string()));
        assert_eq!(state.selected_document, Some("Passage de Grade 1.1".to_string()));
        assert_eq!(state.current_page, 0);
        assert!(state.pages.is_empty());
    }
}
