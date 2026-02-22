use rand::rngs::StdRng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::core::accumulation::AccumulationBuffer;
use crate::core::camera::Camera;
use crate::core::image_buffer::ImageBuffer;
use crate::core::renderer;
use crate::core::renderer::RenderProfile;
use crate::core::scene::Scene;
use crate::presets::quality_profiles::QualityPreset;

use super::reset_policy;
use super::runtime_state::RuntimeState;

/// Number of rows to render per frame update for responsiveness.
const ROWS_PER_CHUNK: u32 = 32;

/// Wraps the two possible RNG types so we can switch at runtime.
enum SessionRng {
    Deterministic(ChaCha8Rng),
    Random(StdRng),
}

/// Orchestrator between app events and the renderer core.
/// Owns accumulation state and decides when to reset.
pub struct RenderSession {
    pub scene: Scene,
    pub state: RuntimeState,
    pub accumulation: AccumulationBuffer,
    pub display: ImageBuffer,
    pub deterministic: bool,
    rng: SessionRng,
    prev_state: RuntimeState,
    /// Current row position within the current sample pass.
    current_row: u32,
}

impl RenderSession {
    pub fn new(
        scene: Scene,
        initial_quality: QualityPreset,
        deterministic: bool,
        width: u32,
        height: u32,
    ) -> Self {
        let state = RuntimeState::new(initial_quality);
        let rng = if deterministic {
            SessionRng::Deterministic(ChaCha8Rng::seed_from_u64(42))
        } else {
            SessionRng::Random(StdRng::from_entropy())
        };

        RenderSession {
            prev_state: state.clone(),
            scene,
            state,
            accumulation: AccumulationBuffer::new(width, height),
            display: ImageBuffer::new(width, height),
            deterministic,
            rng,
            current_row: 0,
        }
    }

    /// Build the active camera from the current camera preset and DOF setting.
    fn build_camera(&self) -> Camera {
        let preset = &self.scene.camera_presets[self.state.camera_preset_index];
        let aspect = self.accumulation.width as f32 / self.accumulation.height as f32;
        Camera::look_at(
            preset.position,
            preset.target,
            preset.vfov_degrees,
            aspect,
            preset.focus_distance,
            self.state.dof.aperture_radius(),
        )
    }

    /// Build the render profile from the current quality preset.
    fn build_render_profile(&self) -> RenderProfile {
        self.state.quality.to_render_profile()
    }

    /// Check if the state changed since last frame and reset accumulation if needed.
    pub fn check_and_reset(&mut self) {
        if reset_policy::needs_reset(&self.prev_state, &self.state) {
            self.accumulation.clear();
            self.current_row = 0;
            if self.deterministic {
                self.rng = SessionRng::Deterministic(ChaCha8Rng::seed_from_u64(42));
            }
            self.prev_state = self.state.clone();
        }
    }

    /// Render one chunk of rows and update the display buffer.
    /// Returns to the caller quickly so the window stays responsive.
    pub fn render_step(&mut self) {
        self.check_and_reset();

        let camera = self.build_camera();
        let profile = self.build_render_profile();
        let height = self.accumulation.height;

        let y_start = self.current_row;
        let y_end = (self.current_row + ROWS_PER_CHUNK).min(height);

        match &mut self.rng {
            SessionRng::Deterministic(rng) => {
                renderer::render_row_chunk(&self.scene, &camera, &profile, &mut self.accumulation, rng, y_start, y_end);
            }
            SessionRng::Random(rng) => {
                renderer::render_row_chunk(&self.scene, &camera, &profile, &mut self.accumulation, rng, y_start, y_end);
            }
        }

        self.current_row = y_end;

        // If we finished a full pass, increment sample count and reset row counter
        if self.current_row >= height {
            self.accumulation.increment_sample_count();
            self.current_row = 0;
        }

        // Update display buffer from accumulation
        self.update_display();
    }

    fn update_display(&mut self) {
        let width = self.display.width;
        let height = self.display.height;
        for y in 0..height {
            for x in 0..width {
                let color = self.accumulation.get_averaged_color(x, y);
                self.display.set_pixel(x, y, color.to_pixel());
            }
        }
    }

    pub fn sample_count(&self) -> u32 {
        self.accumulation.sample_count
    }

    pub fn current_camera_name(&self) -> &str {
        self.scene.camera_presets[self.state.camera_preset_index].name
    }

    pub fn camera_preset_count(&self) -> usize {
        self.scene.camera_presets.len()
    }

    pub fn cycle_quality(&mut self) {
        self.state.quality = self.state.quality.next();
    }

    pub fn cycle_camera(&mut self) {
        self.state.camera_preset_index =
            (self.state.camera_preset_index + 1) % self.scene.camera_presets.len();
    }

    pub fn cycle_dof(&mut self) {
        self.state.dof = self.state.dof.next();
    }
}
