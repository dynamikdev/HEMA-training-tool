//! Tests for the curriculum viewer UI logic.

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use crate::components::*;
    use crate::resources::CurriculumState;
    use crate::ui::curriculum::{toggle_curriculum_visibility, curriculum_keyboard_navigation};

    #[test]
    fn test_toggle_curriculum_visibility() {
        let mut app = App::new();

        // Add required components/resources
        let mut curr_state = CurriculumState::default();
        curr_state.is_visible = true;
        app.insert_resource(curr_state);

        app.add_systems(Update, toggle_curriculum_visibility);

        // Spawn a target circle entity
        let target_entity = app.world_mut().spawn((
            Visibility::Visible,
            NumberIndex(1),
        )).id();

        // Spawn a curriculum viewer entity
        let viewer_entity = app.world_mut().spawn((
            Visibility::Hidden,
            crate::ui::curriculum::CurriculumViewer,
        )).id();

        // Run update to apply visibility toggle
        app.update();

        // Verify visibilities are swapped
        assert_eq!(
            *app.world().get::<Visibility>(target_entity).unwrap(),
            Visibility::Hidden,
            "Target circle should be hidden."
        );
        assert_eq!(
            *app.world().get::<Visibility>(viewer_entity).unwrap(),
            Visibility::Visible,
            "Viewer should be visible."
        );
    }

    #[test]
    fn test_keyboard_navigation() {
        let mut app = App::new();

        let mut input = ButtonInput::<KeyCode>::default();
        input.press(KeyCode::ArrowRight);
        app.insert_resource(input);

        let mut state = CurriculumState::default();
        state.is_visible = true;
        // Mock 3 pages
        state.pages = vec![Handle::default(), Handle::default(), Handle::default()];
        state.current_page = 0;
        app.insert_resource(state);

        app.add_systems(Update, curriculum_keyboard_navigation);

        // Update to trigger right arrow press
        app.update();

        assert_eq!(
            app.world().resource::<CurriculumState>().current_page,
            1,
            "Current page should increment."
        );

        // Simulate left arrow press
        let mut input = ButtonInput::<KeyCode>::default();
        input.press(KeyCode::ArrowLeft);
        app.world_mut().insert_resource(input);

        // Update to trigger left arrow press
        app.update();

        assert_eq!(
            app.world().resource::<CurriculumState>().current_page,
            0,
            "Current page should decrement."
        );
    }
}
