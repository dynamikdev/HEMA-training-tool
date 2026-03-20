//! Tests for the UI setup logic.

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use bevy::post_process::bloom::Bloom;
    use bevy::render::view::Hdr;
    use crate::ui::setup::setup;
    use crate::resources::Typography;

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
}
