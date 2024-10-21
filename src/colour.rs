use crate::vec3::Vec3;

pub struct Colour {
    vec3: Vec3,
}

impl Colour {
    pub fn new(red: f64, green: f64, blue: f64) -> Colour {
        Colour {
            vec3: Vec3 {
                x: red,
                y: green,
                z: blue,
            },
        }
    }

    pub fn from_vec3 (vec3: Vec3) -> Colour {
        Colour {
            vec3
        }
    }
}

pub fn red(c: &Colour) -> f64 {
    c.vec3.x
}
pub fn green(c: &Colour) -> f64 {
    c.vec3.y
}
pub fn blue(c: &Colour) -> f64 {
    c.vec3.z
}
