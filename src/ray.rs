use crate::Point3D;
use crate::Vector3;

#[derive(Default, Debug, Clone, Copy)]
pub struct Ray {
    origin: Point3D,
    direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point3D, direction: Vector3) -> Self {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Point3D {
        self.origin
    }

    pub fn direction(&self) -> Vector3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point3D {
        self.origin + self.direction * t
    }
}
