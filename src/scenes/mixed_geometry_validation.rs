use crate::core::geometry::{SceneObject, Sphere, Triangle};
use crate::core::material::Material;
use crate::core::math::{Color, Vec3};
use crate::core::scene::{CameraPreset, Scene};

/// Scene 2: Mixed Geometry Validation
/// Proves triangle support and mixed geometry (spheres + triangles) under the same renderer.
pub fn create() -> Scene {
    let mut materials = Vec::new();
    let mut objects = Vec::new();

    // 0: Ground
    materials.push(Material::Diffuse {
        albedo: Color::rgb(0.4, 0.4, 0.4),
    });
    objects.push(SceneObject::Sphere(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        0,
    )));

    // 1: Diffuse sphere
    materials.push(Material::Diffuse {
        albedo: Color::rgb(0.7, 0.3, 0.3),
    });
    objects.push(SceneObject::Sphere(Sphere::new(
        Vec3::new(-2.0, 1.0, 0.0),
        1.0,
        1,
    )));

    // 2: Glossy sphere
    materials.push(Material::Glossy {
        albedo: Color::rgb(0.6, 0.6, 0.8),
        roughness: 0.2,
    });
    objects.push(SceneObject::Sphere(Sphere::new(
        Vec3::new(2.0, 1.0, 0.0),
        1.0,
        2,
    )));

    // 3: Triangle material — diffuse blue
    materials.push(Material::Diffuse {
        albedo: Color::rgb(0.2, 0.4, 0.9),
    });

    // Upright triangle (like a fin/wall)
    objects.push(SceneObject::Triangle(Triangle::new(
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 3.0, -1.0),
        Vec3::new(2.0, 0.0, -1.0),
        3,
    )));

    // 4: Triangle material — glossy green
    materials.push(Material::Glossy {
        albedo: Color::rgb(0.3, 0.8, 0.3),
        roughness: 0.1,
    });
    objects.push(SceneObject::Triangle(Triangle::new(
        Vec3::new(-2.0, 0.0, -1.0),
        Vec3::new(-2.0, 3.0, -1.0),
        Vec3::new(0.0, 0.0, -1.0),
        4,
    )));

    // 5: Specular triangle — floor panel
    materials.push(Material::Specular {
        albedo: Color::rgb(0.8, 0.8, 0.8),
    });
    objects.push(SceneObject::Triangle(Triangle::new(
        Vec3::new(-3.0, 0.01, 2.0),
        Vec3::new(3.0, 0.01, 2.0),
        Vec3::new(0.0, 0.01, -1.0),
        5,
    )));

    // 6: Emissive sphere — overhead light
    materials.push(Material::Emissive {
        color: Color::rgb(1.0, 0.95, 0.85),
        intensity: 4.0,
    });
    objects.push(SceneObject::Sphere(Sphere::new(
        Vec3::new(0.0, 5.0, 0.0),
        1.5,
        6,
    )));

    // Additional triangle — a small colored triangle on the ground
    // 7: Triangle material — diffuse orange
    materials.push(Material::Diffuse {
        albedo: Color::rgb(0.9, 0.5, 0.1),
    });
    objects.push(SceneObject::Triangle(Triangle::new(
        Vec3::new(-1.0, 0.02, 3.0),
        Vec3::new(1.0, 0.02, 3.0),
        Vec3::new(0.0, 0.02, 1.5),
        7,
    )));

    let camera_presets = vec![
        CameraPreset {
            name: "Front View",
            position: Vec3::new(0.0, 2.5, 7.0),
            target: Vec3::new(0.0, 1.0, 0.0),
            vfov_degrees: 45.0,
            focus_distance: 7.0,
        },
        CameraPreset {
            name: "Side Angle",
            position: Vec3::new(5.0, 2.0, 4.0),
            target: Vec3::new(0.0, 1.0, 0.0),
            vfov_degrees: 45.0,
            focus_distance: 6.0,
        },
    ];

    Scene {
        id: "mixed_geometry",
        name: "Mixed Geometry Validation",
        objects,
        materials,
        camera_presets,
    }
}
