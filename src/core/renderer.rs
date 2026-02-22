use rand::Rng;

use super::camera::Camera;
use super::math::{Color, Vec3};
use super::scene::Scene;
use super::accumulation::AccumulationBuffer;

/// Render configuration derived from quality profile + DOF preset.
#[derive(Debug, Clone)]
pub struct RenderProfile {
    pub max_bounces: u32,
}

/// Render a chunk of rows [y_start, y_end) for one sample into the accumulation buffer.
/// Returns true if this was the last chunk of the current sample pass.
pub fn render_row_chunk<R: Rng>(
    scene: &Scene,
    camera: &Camera,
    profile: &RenderProfile,
    accumulation: &mut AccumulationBuffer,
    rng: &mut R,
    y_start: u32,
    y_end: u32,
) {
    let width = accumulation.width;
    let height = accumulation.height;

    for y in y_start..y_end.min(height) {
        for x in 0..width {
            let jitter_x: f32 = rng.gen();
            let jitter_y: f32 = rng.gen();

            let ray = camera.get_ray(x, y, width, height, jitter_x, jitter_y, rng);
            let color = trace_ray(scene, &ray, profile.max_bounces, rng);
            accumulation.add_sample(x, y, color);
        }
    }
}

/// Trace a single ray through the scene, accumulating color through bounces.
fn trace_ray<R: Rng>(scene: &Scene, ray: &super::math::Ray, max_bounces: u32, rng: &mut R) -> Color {
    let mut current_ray = *ray;
    let mut accumulated_light = Color::BLACK;
    let mut throughput = Color::WHITE;

    for _bounce in 0..=max_bounces {
        if let Some(hit) = scene.hit(&current_ray, 0.001, f32::MAX) {
            let material = &scene.materials[hit.material_index];

            // Add emitted light
            let emitted = material.emitted();
            accumulated_light += throughput * emitted;

            // Scatter
            if let Some(scatter) = material.scatter(&current_ray, &hit, rng) {
                throughput = throughput * scatter.attenuation;
                current_ray = scatter.scattered_ray;
            } else {
                // Ray absorbed (or emissive-only material)
                break;
            }
        } else {
            // Sky/background contribution
            accumulated_light += throughput * sky_color(&current_ray);
            break;
        }
    }

    accumulated_light
}

/// Simple gradient sky background. Blue to white based on ray direction Y.
fn sky_color(ray: &super::math::Ray) -> Color {
    let unit_dir = ray.direction.normalized();
    let t = 0.5 * (unit_dir.y + 1.0);
    let sky_blue = Color::rgb(0.5, 0.7, 1.0);
    let white = Color::WHITE;
    Vec3::lerp(white, sky_blue, t)
}
