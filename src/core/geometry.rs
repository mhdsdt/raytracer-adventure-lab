use super::math::{Ray, Vec3};

/// Result of a ray-geometry intersection.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material_index: usize,
}

impl HitRecord {
    /// Ensure normal always points against the incoming ray.
    pub fn with_face_normal(t: f32, point: Vec3, outward_normal: Vec3, ray: &Ray, material_index: usize) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            point,
            normal,
            t,
            front_face,
            material_index,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material_index: usize,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material_index: usize) -> Self {
        Sphere {
            center,
            radius,
            material_index,
        }
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root in acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let point = ray.at(root);
        let outward_normal = (point - self.center) / self.radius;
        Some(HitRecord::with_face_normal(
            root,
            point,
            outward_normal,
            ray,
            self.material_index,
        ))
    }
}

#[derive(Debug, Clone)]
pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
    pub material_index: usize,
}

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3, material_index: usize) -> Self {
        Triangle {
            v0,
            v1,
            v2,
            material_index,
        }
    }

    /// Möller–Trumbore intersection
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;
        let h = ray.direction.cross(edge2);
        let a = edge1.dot(h);

        if a.abs() < 1e-7 {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.origin - self.v0;
        let u = f * s.dot(h);
        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let q = s.cross(edge1);
        let v = f * ray.direction.dot(q);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2.dot(q);
        if t < t_min || t > t_max {
            return None;
        }

        let point = ray.at(t);
        let outward_normal = edge1.cross(edge2).normalized();
        Some(HitRecord::with_face_normal(
            t,
            point,
            outward_normal,
            ray,
            self.material_index,
        ))
    }
}

/// A scene object that can be hit by a ray.
#[derive(Debug, Clone)]
pub enum SceneObject {
    Sphere(Sphere),
    Triangle(Triangle),
}

impl SceneObject {
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self {
            SceneObject::Sphere(s) => s.hit(ray, t_min, t_max),
            SceneObject::Triangle(t) => t.hit(ray, t_min, t_max),
        }
    }
}
