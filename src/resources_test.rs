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
}
