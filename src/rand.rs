use crate::Vector3;

pub fn random_f64() -> f64 {
    rand::random::<f64>()
}

pub fn random_f64_range(min: f64, max: f64) -> f64 {
    min + (max - min) * rand::random::<f64>()
}
pub fn random_vec3() -> Vector3 {
    Vector3 {
        x: random_f64(),
        y: random_f64(),
        z: random_f64(),
    }
}

pub fn random_vec3_range(min: f64, max: f64) -> Vector3 {
    Vector3 {
        x: random_f64_range(min, max),
        y: random_f64_range(min, max),
        z: random_f64_range(min, max),
    }
}

pub fn random_vec3_unit_sphere() -> Vector3 {
    let v = random_vec3_range(-1.0, 1.0);
    v.normalized()
}

pub fn random_vec3_unit_hemisphere(normal: Vector3) -> Vector3 {
    let v = random_vec3_unit_sphere();
    if normal.dot(v) >= 0.0 {
        v
    } else {
        -v
    }
}
