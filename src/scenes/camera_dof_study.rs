use crate::core::geometry::{SceneObject, Sphere};
use crate::core::material::Material;
use crate::core::math::{Color, Vec3};
use crate::core::scene::{CameraPreset, Scene};

/// Scene 3: Camera and DOF Study (Deterministic Benchmark)
/// Proves camera preset switching, DOF preset behavior, and deterministic quality comparisons.
/// Objects at varying depths make DOF effect clearly visible.
pub fn create() -> Scene {
    let mut materials = Vec::new();
    let mut objects = Vec::new();

    // 0: Ground
    materials.push(Material::Diffuse {
        albedo: Color::rgb(0.45, 0.45, 0.45),
    });
    objects.push(SceneObject::Sphere(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        0,
    )));

    // Row of spheres at varying depths along Z, centered at X=0
    // Near sphere (z = 2)
    materials.push(Material::Diffuse {
        albedo: Color::rgb(0.9, 0.2, 0.2),
    });
    objects.push(SceneObject::Sphere(Sphere::new(
        Vec3::new(-1.5, 0.5, 2.0),
        0.5,
        1,
    )));

    // Mid-near sphere (z = 0)
    materials.push(Material::Glossy {
        albedo: Color::rgb(0.2, 0.8, 0.2),
        roughness: 0.15,
    });
    objects.push(SceneObject::Sphere(Sphere::new(
        Vec3::new(0.0, 0.7, 0.0),
        0.7,
        2,
    )));

    // Mid sphere (z = -2)
    materials.push(Material::Specular {
        albedo: Color::rgb(0.8, 0.8, 0.9),
    });
    objects.push(SceneObject::Sphere(Sphere::new(
        Vec3::new(1.5, 0.6, -2.0),
        0.6,
        3,
    )));

    // Far sphere (z = -5)
    materials.push(Material::Diffuse {
        albedo: Color::rgb(0.3, 0.3, 0.9),
    });
    objects.push(SceneObject::Sphere(Sphere::new(
        Vec3::new(-0.5, 0.8, -5.0),
        0.8,
        4,
    )));

    // Very far sphere (z = -8)
    materials.push(Material::Glossy {
        albedo: Color::rgb(0.9, 0.7, 0.2),
        roughness: 0.3,
    });
    objects.push(SceneObject::Sphere(Sphere::new(
        Vec3::new(1.0, 1.0, -8.0),
        1.0,
        5,
    )));

    // Emissive light above center
    materials.push(Material::Emissive {
        color: Color::rgb(1.0, 0.95, 0.9),
        intensity: 3.0,
    });
    objects.push(SceneObject::Sphere(Sphere::new(
        Vec3::new(0.0, 6.0, -2.0),
        2.0,
        6,
    )));

    // Small accent spheres for visual interest
    materials.push(Material::Diffuse {
        albedo: Color::rgb(0.8, 0.4, 0.7),
    });
    objects.push(SceneObject::Sphere(Sphere::new(
        Vec3::new(2.5, 0.3, 1.0),
        0.3,
        7,
    )));

    materials.push(Material::Specular {
        albedo: Color::rgb(0.95, 0.95, 0.95),
    });
    objects.push(SceneObject::Sphere(Sphere::new(
        Vec3::new(-2.0, 0.4, -1.0),
        0.4,
        8,
    )));

    let camera_presets = vec![
        CameraPreset {
            name: "Focus Near",
            position: Vec3::new(0.0, 2.0, 6.0),
            target: Vec3::new(0.0, 0.5, 2.0),
            vfov_degrees: 40.0,
            focus_distance: 4.5,
        },
        CameraPreset {
            name: "Focus Mid",
            position: Vec3::new(0.0, 2.0, 6.0),
            target: Vec3::new(0.0, 0.7, 0.0),
            vfov_degrees: 40.0,
            focus_distance: 6.5,
        },
        CameraPreset {
            name: "Focus Far",
            position: Vec3::new(0.0, 2.0, 6.0),
            target: Vec3::new(0.0, 0.8, -5.0),
            vfov_degrees: 40.0,
            focus_distance: 11.0,
        },
        CameraPreset {
            name: "Side Perspective",
            position: Vec3::new(5.0, 1.5, 2.0),
            target: Vec3::new(0.0, 0.7, -2.0),
            vfov_degrees: 45.0,
            focus_distance: 6.0,
        },
    ];

    Scene {
        id: "camera_dof_study",
        name: "Camera & DOF Study",
        objects,
        materials,
        camera_presets,
    }
}
