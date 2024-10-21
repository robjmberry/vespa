
#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn add(self: &Vec3, b: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + b.x,
            y: self.y + b.y,
            z: self.z + b.z,
        }
    }

    pub fn subtract(self: &Vec3, b: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - b.x,
            y: self.y - b.y,
            z: self.z - b.z,
        }
    }

    pub fn multiple_scalar(self: &Vec3, t: f64) -> Vec3 {
        Vec3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }

    pub fn divide_scalar(self: &Vec3, t: f64) -> Vec3 {
        self.multiple_scalar(1. / t)
    }

    pub fn length (self: &Vec3) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared (self: &Vec3) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unit_vector (self: &Vec3) -> Vec3 {
        self.divide_scalar(self.length())
    }

    pub fn dot (self: &Vec3, b: &Vec3) -> f64 {
        let x = self.x * b.x;
        let y = self.y * b.y;
        let z = self.z  * b.z;
        x+y+z
    }
}

pub type Point3 = Vec3;
