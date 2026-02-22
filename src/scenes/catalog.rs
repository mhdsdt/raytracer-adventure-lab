use crate::core::scene::Scene;

use super::camera_dof_study;
use super::materials_lighting_showcase;
use super::mixed_geometry_validation;

/// All official Phase 1 demo scenes.
pub fn all_scenes() -> Vec<Scene> {
    vec![
        materials_lighting_showcase::create(),
        mixed_geometry_validation::create(),
        camera_dof_study::create(),
    ]
}

/// Find a scene by its ID. Returns the default scene if not found.
pub fn find_scene(id: &str) -> Option<Scene> {
    all_scenes().into_iter().find(|s| s.id == id)
}

/// Returns the default scene (first in catalog).
#[allow(dead_code)]
pub fn default_scene() -> Scene {
    materials_lighting_showcase::create()
}

/// List all available scene IDs.
pub fn scene_ids() -> Vec<&'static str> {
    vec!["materials_lighting", "mixed_geometry", "camera_dof_study"]
}
