//! Tests for the UI setup logic.

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use bevy::post_process::bloom::Bloom;
    use bevy::render::view::Hdr;
    use crate::ui::setup::setup;
    use crate::resources::{CurriculumState, SequenceState, Typography};
    use crate::components::CurriculumToggleButton;
    use crate::ui::systems::curriculum_toggle_system;

    /// Verifies that clicking the Curriculum Toggle Button toggles visibility and pauses the sequence.
    #[test]
    fn test_curriculum_toggle_system() {
        let mut app = App::new();
        app.insert_resource(CurriculumState::default());
        app.insert_resource(SequenceState { running: true, ..default() });
        app.add_systems(Update, curriculum_toggle_system);

        let entity = app.world_mut().spawn((
            CurriculumToggleButton,
            Interaction::Pressed,
            BackgroundColor(Color::NONE),
        )).id();
        let text_entity = app.world_mut().spawn(Text::new("Open Curriculum")).id();
        app.world_mut().entity_mut(entity).add_child(text_entity);

        app.update();

        let curr_state = app.world().resource::<CurriculumState>();
        let seq_state = app.world().resource::<SequenceState>();

        assert!(curr_state.is_visible, "Curriculum should become visible.");
        assert!(!seq_state.running, "Sequence should be paused when curriculum is opened.");
    }

    /// Verifies that the camera is spawned with the Hdr and Bloom components.
    #[test]
    fn test_camera_has_hdr_and_bloom() {
        let mut app = App::new();
        app.insert_resource(Typography::default());
        app.add_plugins((AssetPlugin::default(), bevy::text::TextPlugin::default()));
        app.add_systems(Startup, setup);
        app.update();

        // Check for camera with Hdr and Bloom
        let mut query = app.world_mut().query::<(&Camera, &Hdr, &Bloom)>();
        let _ = query.single(app.world()).expect("Camera with Hdr and Bloom not found");
    }

    /// Verifies that the target numbers are spawned with the correct Z-coordinate and ZIndex.
    #[test]
    fn test_target_numbers_layering() {
        let mut app = App::new();
        app.insert_resource(Typography::default());
        app.add_plugins((AssetPlugin::default(), bevy::text::TextPlugin::default()));
        app.add_systems(Startup, setup);
        app.update();

        use crate::components::NumberIndex;
        let mut query = app.world_mut().query::<(&NumberIndex, &Transform, &ZIndex)>();
        
        let count = query.iter(app.world()).count();
        assert_eq!(count, 8, "Expected 8 target numbers to be spawned");

        for (_index, transform, z_index) in query.iter(app.world()) {
            assert_eq!(transform.translation.z, 1.0, "Target number Z-coordinate must be 1.0");
            assert_eq!(z_index.0, 1, "Target number ZIndex must be 1");
        }
    }
}
