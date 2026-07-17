#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use crate::ui::systems::*;
    use crate::resources::*;
    use crate::components::*;

    #[test]
    fn test_sync_curriculum_ui_labels() {
        let mut app = App::new();

        app.insert_resource(CurriculumState::default());

        app.add_systems(Update, sync_curriculum_ui_labels);

        // create entities with the correct text and marker components
        let grade_entity = app.world_mut().spawn((
            Text::new(""),
            ParentButton(std::marker::PhantomData::<CurriculumGradeButton>),
        )).id();

        let doc_entity = app.world_mut().spawn((
            Text::new(""),
            ParentButton(std::marker::PhantomData::<CurriculumDocumentButton>),
        )).id();

        let page_entity = app.world_mut().spawn((
            Text::new(""),
            CurriculumPageText,
        )).id();

        app.update();

        // Validate defaults
        assert_eq!(app.world().get::<Text>(grade_entity).unwrap().0, "Grade: Niveau 1.1");
        assert_eq!(app.world().get::<Text>(doc_entity).unwrap().0, "Doc: Passage de Grade 1.1");
        assert_eq!(app.world().get::<Text>(page_entity).unwrap().0, "Page: -- / --");

        // Mutate CurriculumState
        let mut state = app.world_mut().resource_mut::<CurriculumState>();
        state.selected_grade = Some("Niveau 1.2".to_string());
        state.selected_document = Some("Passage de grade 1.2".to_string());
        state.current_page = 2;
        state.pages = vec![Handle::default(), Handle::default(), Handle::default(), Handle::default()];

        app.update();

        assert_eq!(app.world().get::<Text>(grade_entity).unwrap().0, "Grade: Niveau 1.2");
        assert_eq!(app.world().get::<Text>(doc_entity).unwrap().0, "Doc: Passage de grade 1.2");
        assert_eq!(app.world().get::<Text>(page_entity).unwrap().0, "Page: 3 / 4");
    }
}
