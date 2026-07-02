//! Tests for the target visualization logic.

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use std::f32::consts::PI;
    use crate::ui::target::spawn_target_circle;
    use crate::components::NumberIndex;
    use crate::constants::{CIRCLE_RADIUS, LABELS};
    use crate::resources::Typography;

    fn test_setup_system(mut commands: Commands, typography: Res<Typography>) {
        spawn_target_circle(&mut commands, &typography);
    }

    #[test]
    fn test_spawn_target_circle() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(bevy::asset::AssetPlugin::default());
        app.insert_resource(Typography::default());
        app.add_systems(Startup, test_setup_system);
        app.update();

        let mut query = app.world_mut().query::<(&NumberIndex, &Transform, &Text2d, &Name)>();
        let count = query.iter(app.world()).count();
        assert_eq!(count, 8, "Expected 8 target entities to be spawned");

        for (index, transform, text, name) in query.iter(app.world()) {
            let i = index.0;
            let expected_label = LABELS[i as usize];

            assert_eq!(text.0, format!("{}", expected_label));
            assert_eq!(name.as_str(), format!("Target {}", expected_label));

            let angle = PI / 2.0 - (i as f32) * (PI / 4.0);
            let expected_x = angle.cos() * CIRCLE_RADIUS;
            let expected_y = angle.sin() * CIRCLE_RADIUS;

            assert!((transform.translation.x - expected_x).abs() < 0.001);
            assert!((transform.translation.y - expected_y).abs() < 0.001);
            assert_eq!(transform.translation.z, 1.0);
        }
    }
}
