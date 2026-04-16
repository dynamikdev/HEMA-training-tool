use bevy::prelude::*;
use bevy::app::TaskPoolPlugin;
use crate::components::CurriculumGradeButton;
use crate::resources::CurriculumState;
use crate::ui::systems::curriculum_grade_system;

#[test]
fn test_curriculum_grade_system() {
    let mut app = App::new();
    app.add_plugins(TaskPoolPlugin::default());

    // Setup initial state
    let mut state = CurriculumState::default();
    state.selected_grade = Some("Niveau 1.1".to_string());
    state.selected_document = Some("Passage de Grade 1.1".to_string());
    app.insert_resource(state);

    // Spawn button with pressed interaction
    app.world_mut().spawn((
        CurriculumGradeButton,
        Interaction::Pressed,
    ));

    app.add_systems(Update, curriculum_grade_system);

    // Run app to process the interaction
    app.update();

    let new_state = app.world().resource::<CurriculumState>();

    // According to CURRICULUM_MANIFEST, the grades sorted are "Niveau 1.1" and "Niveau 1.2"
    // So if current is "Niveau 1.1", next should be "Niveau 1.2"
    assert_eq!(new_state.selected_grade, Some("Niveau 1.2".to_string()));
    assert_eq!(new_state.selected_document, Some("Passage de grade 1.2".to_string()));
}
