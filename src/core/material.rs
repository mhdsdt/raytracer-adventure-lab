use rand::Rng;

use super::math::{Color, Ray, Vec3};
use super::geometry::HitRecord;
use super::sampling;

/// Material types supported in Phase 1.
#[derive(Debug, Clone)]
pub enum Material {
    /// Lambertian diffuse: scatters in random hemisphere direction.
    Diffuse { albedo: Color },
    /// Perfect specular mirror reflection.
    Specular { albedo: Color },
    /// Glossy: interpolates between specular reflection and diffuse scatter.
    /// `roughness` in [0, 1]: 0 = mirror, 1 = fully diffuse.
    Glossy { albedo: Color, roughness: f32 },
    /// Emissive: emits light with given color and intensity.
    Emissive { color: Color, intensity: f32 },
}

/// Result of a material scatter event.
pub struct ScatterResult {
    pub scattered_ray: Ray,
    pub attenuation: Color,
}

impl Material {
    /// Returns the emitted light from this material (zero for non-emissive).
    pub fn emitted(&self) -> Color {
        match self {
            Material::Emissive { color, intensity } => *color * *intensity,
            _ => Color::BLACK,
        }
    }

    /// Scatter an incoming ray at a hit point, returning the scattered ray and color attenuation.
    pub fn scatter<R: Rng>(&self, ray: &Ray, hit: &HitRecord, rng: &mut R) -> Option<ScatterResult> {
        match self {
            Material::Diffuse { albedo } => {
                let scatter_dir = sampling::random_hemisphere_direction(hit.normal, rng);
                Some(ScatterResult {
                    scattered_ray: Ray::new(hit.point + hit.normal * 1e-4, scatter_dir),
                    attenuation: *albedo,
                })
            }
            Material::Specular { albedo } => {
                let reflected = ray.direction.normalized().reflect(hit.normal);
                if reflected.dot(hit.normal) > 0.0 {
                    Some(ScatterResult {
                        scattered_ray: Ray::new(hit.point + hit.normal * 1e-4, reflected),
                        attenuation: *albedo,
                    })
                } else {
                    None
                }
            }
            Material::Glossy { albedo, roughness } => {
                let reflected = ray.direction.normalized().reflect(hit.normal);
                let diffuse_dir = sampling::random_hemisphere_direction(hit.normal, rng);
                // Lerp between perfect reflection and diffuse based on roughness
                let scatter_dir = Vec3::lerp(reflected, diffuse_dir, *roughness).normalized();
                if scatter_dir.dot(hit.normal) > 0.0 {
                    Some(ScatterResult {
                        scattered_ray: Ray::new(hit.point + hit.normal * 1e-4, scatter_dir),
                        attenuation: *albedo,
                    })
                } else {
                    // Fallback: use the diffuse direction if lerped direction goes below surface
                    Some(ScatterResult {
                        scattered_ray: Ray::new(hit.point + hit.normal * 1e-4, diffuse_dir),
                        attenuation: *albedo,
                    })
                }
            }
            Material::Emissive { .. } => None,
        }
    }
}
