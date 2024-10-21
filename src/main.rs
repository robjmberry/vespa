use colour::Colour;
use ppm::PpmFile;
use ray::Ray;
use vec3::{Point3, Vec3};

mod colour;
mod ppm;
mod vec3;

mod ray {
    use crate::vec3::{Point3, Vec3};

    pub struct Ray {
        pub origin: Point3,
        pub direction: Vec3,
    }

    impl Ray {
        pub fn new(origin: Point3, direction: Vec3) -> Ray {
            Ray { origin, direction }
        }
    }

    pub fn at(ray: &Ray, t: f64) -> Point3 {
        let v = &ray.origin.add(&ray.direction);
        v.multiple_scalar(t)
    }
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let image_height = image_width as f64 / aspect_ratio;
    let image_height = if image_height < 1.0 {
        1
    } else {
        image_height as u32
    };

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0., 0., 0.);

    let viewport_u = Vec3::new(viewport_width, 0., 0.);
    let viewport_v = Vec3::new(0., -viewport_height, 0.);

    let pixel_delta_u = viewport_u.divide_scalar(image_width as f64);
    let pixel_delta_v = viewport_v.divide_scalar(image_height as f64);

    let viewport_upper_left =
        calc_viewport_upper_left(&camera_center, focal_length, &viewport_u, &viewport_v);

    let first_pixel = pixel_delta_u.add(&pixel_delta_v).multiple_scalar(0.5);
    let first_pixel = viewport_upper_left.add(&first_pixel);

    let mut colours = Vec::new();
    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center = first_pixel
                .add(&pixel_delta_u.multiple_scalar(i as f64))
                .add(&pixel_delta_v.multiple_scalar(j as f64));
            let ray_direction = pixel_center.subtract(&camera_center);

            let ray = Ray::new(camera_center, ray_direction);
            colours.push(ray_colour(&ray));
        }
    }

    let output = PpmFile::new(String::from("render.ppm"), image_height, image_width);
    ppm::write_to_file(output, colours);
}

fn calc_viewport_upper_left(camera_center: &Point3, focal_length: f64, u: &Vec3, v: &Vec3) -> Vec3 {
    let vp_center = camera_center.subtract(&Vec3::new(0., 0., focal_length));
    let half_u = u.multiple_scalar(0.5);
    let half_v = v.multiple_scalar(0.5);

    vp_center.subtract(&half_u).subtract(&half_v)
}


fn ray_colour(ray: &Ray) -> Colour {
    if hit_sphere(Point3::new(0.0, 0.0, -1.1), 0.5, ray) {
        Colour::new(1.0,0.0,0.0)
    }
    else {

    let start = Vec3::new(1., 1., 1.);
    let end = Vec3::new(0.5,0.70,1.0);
    let unit_direction = ray.direction.unit_vector();
    let a = 0.5* (unit_direction.y + 1.0);
    Colour::from_vec3( start.multiple_scalar(1.0-a).add(&end.multiple_scalar(a)))
    }
}

fn hit_sphere (center: Vec3, radius: f64, ray: &Ray) -> bool {
    let oc = center.subtract(&ray.origin);
    let a = ray.direction.dot(&ray.direction);
    let raydot = ray.direction.dot(&ray.direction);
    let b = -2.0 * ray.direction.dot(&oc) ;
    let dot = oc.dot(&oc);
    let c = oc.dot(&oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    // println!("{discriminant}");
    discriminant as i32 >= 0
}
