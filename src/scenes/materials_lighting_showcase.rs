use crate::core::geometry::{SceneObject, Sphere};
use crate::core::material::Material;
use crate::core::math::{Color, Vec3};
use crate::core::scene::{CameraPreset, Scene};

/// Scene 1: Materials and Lighting Showcase
/// Demonstrates spheres, material variation, emissive lighting, sky background, progressive refinement.
pub fn create() -> Scene {
    let mut materials = Vec::new();
    let mut objects = Vec::new();

    // 0: Ground — large diffuse grey sphere
    materials.push(Material::Diffuse {
        albedo: Color::rgb(0.5, 0.5, 0.5),
    });
    objects.push(SceneObject::Sphere(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        0,
    )));

    // 1: Center — diffuse red
    materials.push(Material::Diffuse {
        albedo: Color::rgb(0.8, 0.2, 0.2),
    });
    objects.push(SceneObject::Sphere(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        1,
    )));

    // 2: Left — specular mirror
    materials.push(Material::Specular {
        albedo: Color::rgb(0.9, 0.9, 0.9),
    });
    objects.push(SceneObject::Sphere(Sphere::new(
        Vec3::new(-2.5, 1.0, 0.0),
        1.0,
        2,
    )));

    // 3: Right — glossy blue
    materials.push(Material::Glossy {
        albedo: Color::rgb(0.3, 0.3, 0.8),
        roughness: 0.3,
    });
    objects.push(SceneObject::Sphere(Sphere::new(
        Vec3::new(2.5, 1.0, 0.0),
        1.0,
        3,
    )));

    // 4: Emissive light sphere (warm light)
    materials.push(Material::Emissive {
        color: Color::rgb(1.0, 0.9, 0.7),
        intensity: 5.0,
    });
    objects.push(SceneObject::Sphere(Sphere::new(
        Vec3::new(0.0, 4.0, -2.0),
        1.0,
        4,
    )));

    // 5: Small glossy green sphere in front
    materials.push(Material::Glossy {
        albedo: Color::rgb(0.2, 0.7, 0.3),
        roughness: 0.15,
    });
    objects.push(SceneObject::Sphere(Sphere::new(
        Vec3::new(-1.0, 0.4, 2.0),
        0.4,
        5,
    )));

    // 6: Small diffuse yellow sphere
    materials.push(Material::Diffuse {
        albedo: Color::rgb(0.9, 0.8, 0.2),
    });
    objects.push(SceneObject::Sphere(Sphere::new(
        Vec3::new(1.2, 0.35, 1.8),
        0.35,
        6,
    )));

    // 7: Highly reflective small sphere
    materials.push(Material::Specular {
        albedo: Color::rgb(1.0, 0.8, 0.6),
    });
    objects.push(SceneObject::Sphere(Sphere::new(
        Vec3::new(0.5, 0.25, 3.0),
        0.25,
        7,
    )));

    let camera_presets = vec![
        CameraPreset {
            name: "Front Overview",
            position: Vec3::new(0.0, 2.5, 8.0),
            target: Vec3::new(0.0, 1.0, 0.0),
            vfov_degrees: 40.0,
            focus_distance: 8.0,
        },
        CameraPreset {
            name: "Low Angle",
            position: Vec3::new(3.0, 0.5, 5.0),
            target: Vec3::new(0.0, 1.0, 0.0),
            vfov_degrees: 50.0,
            focus_distance: 5.5,
        },
        CameraPreset {
            name: "Close Up",
            position: Vec3::new(-1.5, 1.5, 3.5),
            target: Vec3::new(0.0, 1.0, 0.0),
            vfov_degrees: 35.0,
            focus_distance: 4.0,
        },
    ];

    Scene {
        id: "materials_lighting",
        name: "Materials & Lighting Showcase",
        objects,
        materials,
        camera_presets,
    }
}
