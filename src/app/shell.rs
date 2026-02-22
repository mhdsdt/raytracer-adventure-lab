use minifb::{Key, Window, WindowOptions};

use super::cli::LaunchConfig;
use super::overlay::Overlay;
use super::render_session::RenderSession;
use crate::scenes::catalog;

/// Run the main app loop: window + input + progressive rendering.
pub fn run(config: LaunchConfig) -> anyhow::Result<()> {
    let scene = catalog::find_scene(&config.scene_id)
        .ok_or_else(|| anyhow::anyhow!("Scene '{}' not found", config.scene_id))?;

    let mut session = RenderSession::new(
        scene,
        config.initial_quality,
        config.deterministic,
        config.width,
        config.height,
    );

    let mut window = Window::new(
        &format!("Ray Tracer — {}", session.scene.name),
        config.width as usize,
        config.height as usize,
        WindowOptions {
            resize: false,
            ..WindowOptions::default()
        },
    )
    .map_err(|e| anyhow::anyhow!("Failed to create window: {}", e))?;

    // Limit frame rate to ~60fps for input polling (rendering is the bottleneck anyway)
    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Handle input
        handle_input(&window, &mut session);

        // Render one incremental step
        session.render_step();

        // Copy display buffer and draw overlay
        let mut frame = session.display.pixels.clone();
        Overlay::render(&session, &mut frame, config.width);

        // Present
        window
            .update_with_buffer(&frame, config.width as usize, config.height as usize)
            .map_err(|e| anyhow::anyhow!("Display error: {}", e))?;
    }

    Ok(())
}

fn handle_input(window: &Window, session: &mut RenderSession) {
    if window.is_key_pressed(Key::Q, minifb::KeyRepeat::No) {
        session.cycle_quality();
    }
    if window.is_key_pressed(Key::C, minifb::KeyRepeat::No) {
        session.cycle_camera();
    }
    if window.is_key_pressed(Key::D, minifb::KeyRepeat::No) {
        session.cycle_dof();
    }
}
