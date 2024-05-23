use apolaki_canvas::Canvas;
use apolaki_color::Color;
use apolaki_light::Light;
use apolaki_material::Material;
use apolaki_objects::{Intersect, Sphere};
use apolaki_ray::Ray;
use apolaki_tuple::point;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), std::io::Error> {
    let mut s = Sphere::default();
    let mut m = Material::default();
    m.color = Color::new(1, 0.2, 1);
    s.material = m;
    let light = Light::new(point(-10, 10, -10), Color::new(1, 1, 1));
    let canvas_pixels = 100;
    let mut canvas = Canvas::with_size(canvas_pixels, canvas_pixels);
    let wall_size = 7.0;
    let wall_z = 10;

    let pixel_size = wall_size / canvas_pixels as f64;

    let half = wall_size / 2.;

    let ray_origin = point(0, 0, -5);
    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * (y as f64);
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * (x as f64);
            let pos = point(world_x, world_y, wall_z);
            let r = Ray::new(ray_origin, (pos - ray_origin).normalize());
            if let Some(its) = s.intersect(r) {
                if let Some(it) = its.hit() {
                    let p = r.position(it.t);
                    let n = it.obj.normal_at(p);
                    let eye = -r.dir;

                    let calculated_color = it.obj.material.lighting(&light, p, eye, n);
                    canvas.write(x, y, calculated_color);
                }
            }
        }
    }

    let ppm = canvas.to_ppm_string();
    let mut f = File::create("pit6.ppm")?;
    f.write(ppm.as_bytes()).and(Ok(()))
}
