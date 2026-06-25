//! Tests for the settings UI logic.

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use bevy_ui_widgets::SliderPlugin;
    use crate::ui::settings::spawn_settings_panel;
    use crate::resources::Typography;

    #[test]
    fn test_spawn_settings_panel() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(SliderPlugin);

        app.insert_resource(Typography::default());

        app.add_systems(Startup, |mut commands: Commands, typography: Res<Typography>| {
            commands.spawn(Node::default()).with_children(|parent| {
                spawn_settings_panel(parent, &typography);
            });
        });

        app.update();

        let mut text_query = app.world_mut().query::<&Text>();

        let mut found_settings = false;
        let mut found_mode = false;
        let mut found_sequence = false;
        let mut found_rhythm = false;
        let mut found_curriculum = false;

        for text in text_query.iter(app.world()) {
            if text.0 == "SETTINGS" {
                found_settings = true;
            } else if text.0 == "Mode: Random" {
                found_mode = true;
            } else if text.0 == "Launch Sequence" {
                found_sequence = true;
            } else if text.0 == "Rhythm: Constant" {
                found_rhythm = true;
            } else if text.0 == "CURRICULUM" {
                found_curriculum = true;
            }
        }

        assert!(found_settings, "Settings panel title not found");
        assert!(found_mode, "Mode toggle button not found");
        assert!(found_sequence, "Sequence control button not found");
        assert!(found_rhythm, "Rhythm control button not found");
        assert!(found_curriculum, "Curriculum section not found");
    }
}
