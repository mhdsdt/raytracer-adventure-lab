use rand::Rng;

use super::math::{Ray, Vec3};

/// Camera definition with position, orientation, FOV, and optional depth of field.
#[derive(Debug, Clone)]
pub struct Camera {
    pub position: Vec3,
    pub forward: Vec3,
    pub right: Vec3,
    pub up: Vec3,
    pub fov_radians: f32,
    pub aspect_ratio: f32,
    /// Distance to the focus plane (for DOF). Objects at this distance are sharp.
    pub focus_distance: f32,
    /// Aperture radius. 0 = pinhole (no DOF blur).
    pub aperture_radius: f32,
}

impl Camera {
    /// Create a camera looking from `position` toward `target`, with vertical FOV in degrees.
    pub fn look_at(
        position: Vec3,
        target: Vec3,
        vfov_degrees: f32,
        aspect_ratio: f32,
        focus_distance: f32,
        aperture_radius: f32,
    ) -> Self {
        let world_up = Vec3::UP;
        let forward = (target - position).normalized();
        let right = forward.cross(world_up).normalized();
        let up = right.cross(forward).normalized();

        Camera {
            position,
            forward,
            right,
            up,
            fov_radians: vfov_degrees.to_radians(),
            aspect_ratio,
            focus_distance,
            aperture_radius,
        }
    }

    /// Generate a ray for pixel (px, py) within image of given dimensions.
    /// `jitter_x` and `jitter_y` are sub-pixel offsets in [0, 1) for anti-aliasing.
    pub fn get_ray<R: Rng>(
        &self,
        px: u32,
        py: u32,
        width: u32,
        height: u32,
        jitter_x: f32,
        jitter_y: f32,
        rng: &mut R,
    ) -> Ray {
        let half_height = (self.fov_radians / 2.0).tan();
        let half_width = half_height * self.aspect_ratio;

        // Normalized device coordinates [-1, 1]
        let u = (2.0 * (px as f32 + jitter_x) / width as f32 - 1.0) * half_width;
        let v = (1.0 - 2.0 * (py as f32 + jitter_y) / height as f32) * half_height;

        let ray_dir = (self.forward + self.right * u + self.up * v).normalized();

        if self.aperture_radius <= 0.0 {
            // Pinhole camera — no DOF
            Ray::new(self.position, ray_dir)
        } else {
            // Thin lens DOF: offset origin on the lens disk
            let focus_point = self.position + ray_dir * (self.focus_distance / ray_dir.dot(self.forward));
            let (lens_dx, lens_dy) = random_in_disk(rng);
            let lens_offset =
                self.right * (lens_dx * self.aperture_radius) + self.up * (lens_dy * self.aperture_radius);
            let new_origin = self.position + lens_offset;
            let new_dir = (focus_point - new_origin).normalized();
            Ray::new(new_origin, new_dir)
        }
    }
}

fn random_in_disk<R: Rng>(rng: &mut R) -> (f32, f32) {
    loop {
        let x = rng.gen_range(-1.0..1.0_f32);
        let y = rng.gen_range(-1.0..1.0_f32);
        if x * x + y * y < 1.0 {
            return (x, y);
        }
    }
}
