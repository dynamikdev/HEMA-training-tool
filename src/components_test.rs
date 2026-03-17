
#[cfg(test)]
mod tests {
    use crate::components::*;

    #[test]
    fn test_number_index_component() {
        let index = NumberIndex(5);
        assert_eq!(index.0, 5);
    }

    #[test]
    fn test_marker_components_exist() {
        // These are markers, so we just check if they can be instantiated.
        let _ = SequenceControlButton;
        let _ = SequenceModeButton;
        let _ = RhythmModeButton;
        let _ = RhythmText;
    }
}
