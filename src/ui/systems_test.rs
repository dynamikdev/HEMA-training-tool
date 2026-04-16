#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use crate::components::*;
    use crate::resources::CurriculumState;
    use crate::ui::systems::curriculum_document_system;

    #[test]
    fn test_curriculum_document_system() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, curriculum_document_system);

        let mut curr_state = CurriculumState::default();
        curr_state.selected_grade = Some("Test Grade".to_string());
        curr_state.selected_document = Some("Doc A".to_string());
        app.insert_resource(curr_state);

        let _entity = app.world_mut().spawn((
            Interaction::Pressed,
            CurriculumDocumentButton,
        )).id();

        app.update();

        let curr_state = app.world().resource::<CurriculumState>();
        // Ensure that the document successfully cycles to the next one in the manifest
        assert_eq!(curr_state.selected_document, Some("Doc B".to_string()));
    }
}
