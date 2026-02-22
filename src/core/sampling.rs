use rand::Rng;

use super::math::Vec3;

/// Generate a random unit vector in the hemisphere aligned with `normal`.
pub fn random_hemisphere_direction<R: Rng>(normal: Vec3, rng: &mut R) -> Vec3 {
    let dir = random_unit_vector(rng);
    if dir.dot(normal) > 0.0 {
        dir
    } else {
        -dir
    }
}

/// Generate a random point on the unit sphere (uniform distribution).
fn random_unit_vector<R: Rng>(rng: &mut R) -> Vec3 {
    loop {
        let v = Vec3::new(
            rng.gen_range(-1.0..1.0_f32),
            rng.gen_range(-1.0..1.0_f32),
            rng.gen_range(-1.0..1.0_f32),
        );
        let len_sq = v.length_squared();
        if len_sq > 1e-6 && len_sq <= 1.0 {
            return v / len_sq.sqrt();
        }
    }
}
