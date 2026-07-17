use bevy::prelude::*;
use crate::components::{CurriculumPrevPageButton, CurriculumNextPageButton};
use crate::resources::CurriculumState;
use super::systems::curriculum_page_button_system;

fn setup_app(is_visible: bool, pages_count: usize, current_page: usize) -> App {
    let mut app = App::new();
    app.add_plugins(bevy::prelude::TaskPoolPlugin::default());

    let mut state = CurriculumState::default();
    state.is_visible = is_visible;
    state.current_page = current_page;
    for _ in 0..pages_count {
        state.pages.push(Handle::default());
    }
    app.insert_resource(state);
    app.add_systems(Update, curriculum_page_button_system);
    app
}

#[test]
fn test_hidden_or_empty_no_effect() {
    let mut app = setup_app(false, 2, 0); // hidden
    app.world_mut().spawn((Interaction::Pressed, CurriculumNextPageButton));
    app.update();
    assert_eq!(app.world().resource::<CurriculumState>().current_page, 0);

    let mut app = setup_app(true, 0, 0); // empty
    app.world_mut().spawn((Interaction::Pressed, CurriculumNextPageButton));
    app.update();
    assert_eq!(app.world().resource::<CurriculumState>().current_page, 0);
}

#[test]
fn test_prev_button_decrements() {
    let mut app = setup_app(true, 3, 1);
    app.world_mut().spawn((Interaction::Pressed, CurriculumPrevPageButton));
    app.update();
    assert_eq!(app.world().resource::<CurriculumState>().current_page, 0);
}

#[test]
fn test_prev_button_at_zero() {
    let mut app = setup_app(true, 3, 0);
    app.world_mut().spawn((Interaction::Pressed, CurriculumPrevPageButton));
    app.update();
    assert_eq!(app.world().resource::<CurriculumState>().current_page, 0);
}

#[test]
fn test_next_button_increments() {
    let mut app = setup_app(true, 3, 1);
    app.world_mut().spawn((Interaction::Pressed, CurriculumNextPageButton));
    app.update();
    assert_eq!(app.world().resource::<CurriculumState>().current_page, 2);
}

#[test]
fn test_next_button_at_max() {
    let mut app = setup_app(true, 3, 2);
    app.world_mut().spawn((Interaction::Pressed, CurriculumNextPageButton));
    app.update();
    assert_eq!(app.world().resource::<CurriculumState>().current_page, 2);
}

#[test]
fn test_ignore_hover_none() {
    let mut app = setup_app(true, 3, 1);
    app.world_mut().spawn((Interaction::Hovered, CurriculumNextPageButton));
    app.world_mut().spawn((Interaction::None, CurriculumPrevPageButton));
    app.update();
    assert_eq!(app.world().resource::<CurriculumState>().current_page, 1);
}
