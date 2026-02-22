use super::geometry::{HitRecord, SceneObject};
use super::material::Material;
use super::math::Ray;

/// A complete scene: objects, materials, and metadata.
#[derive(Debug, Clone)]
pub struct Scene {
    pub id: &'static str,
    pub name: &'static str,
    pub objects: Vec<SceneObject>,
    pub materials: Vec<Material>,
    pub camera_presets: Vec<CameraPreset>,
}

/// A named camera preset within a scene.
#[derive(Debug, Clone)]
pub struct CameraPreset {
    pub name: &'static str,
    pub position: super::math::Vec3,
    pub target: super::math::Vec3,
    pub vfov_degrees: f32,
    /// Default focus distance for this camera.
    pub focus_distance: f32,
}

impl Scene {
    /// Find the closest intersection of a ray with any object in the scene.
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest = t_max;
        let mut best_hit: Option<HitRecord> = None;

        for obj in &self.objects {
            if let Some(hit) = obj.hit(ray, t_min, closest) {
                closest = hit.t;
                best_hit = Some(hit);
            }
        }

        best_hit
    }
}
