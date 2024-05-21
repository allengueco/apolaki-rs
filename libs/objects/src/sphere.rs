use apolaki_material::Material;
use apolaki_matrix::{BaseMatrix, Invert};
use apolaki_ray::Ray;
use apolaki_transform::Transform;
use apolaki_tuple::{point, Tuple};

use crate::intersect::{Intersect, Intersection, Intersections};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    pub radius: f64,
    pub transform: BaseMatrix<4>,
    pub material: Material,
}

impl Sphere {
    pub fn new(radius: f64) -> Self {
        Self {
            radius,
            transform: BaseMatrix::identity(),
            ..Default::default()
        }
    }

    pub fn normal_at(&self, at: Tuple) -> Tuple {
        if at.is_vec() {
            panic!("parameter `at` must be a point")
        }

        let object_normal = {
            let object_point = self.transform.invert() * at;
            object_point - point(0, 0, 0)
        };

        let mut world_normal = self.transform.invert().transpose() * object_normal;
        world_normal.set_w(0.);

        world_normal.normalize()
    }

    pub fn transform(&mut self, transform: BaseMatrix<4>) {
        self.transform = self.transform * transform;
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            radius: 1.0,
            transform: BaseMatrix::identity(),
            material: Material::default(),
        }
    }
}

impl Intersect for Sphere {
    type O = Self;
    fn intersect(&self, ray: Ray) -> Option<Intersections<Self::O>> {
        let r = ray.transform(self.transform.invert());
        let sphere_to_ray = r.origin - point(0, 0, 0);
        let a = r.dir.dot(r.dir);
        let b = 2. * r.dir.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.;

        let discriminant = b.powi(2) - 4. * a * c;

        if discriminant < 0. {
            None
        } else {
            Some(Intersections::from(Vec::from([
                Intersection::new((-b - discriminant.sqrt()) / (2. * a), *self),
                Intersection::new((-b + discriminant.sqrt()) / (2. * a), *self),
            ])))
        }
    }
}

#[cfg(test)]
mod sphere_tests {
    use std::f32::consts::FRAC_1_SQRT_2;
    use std::f64::consts::{PI, SQRT_2};

    use apolaki_ray::Ray;
    use apolaki_tuple::{point, vector};

    use super::*;

    #[test]
    fn ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(point(0, 0, -5), vector(0, 0, 1));
        let s = Sphere::default();
        let xs = s.intersect(r).expect("must have intersections");
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn ray_intersects_a_sphere_at_tangent() {
        let r = Ray::new(point(0, 1, -5), vector(0, 0, 1));
        let s = Sphere::default();
        let xs = s.intersect(r).expect("must have intersections");
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn ray_misses_a_sphere() {
        let r = Ray::new(point(0, 2, -5), vector(0, 0, 1));
        let s = Sphere::default();
        let xs = s.intersect(r);
        assert_eq!(xs, None);
    }

    #[test]
    fn ray_originates_inside() {
        let r = Ray::new(point(0, 0, 0), vector(0, 0, 1));
        let s = Sphere::default();
        let xs = s.intersect(r).expect("must have intersections");
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn sphere_is_behind_ray() {
        let r = Ray::new(point(0, 0, 5), vector(0, 0, 1));
        let s = Sphere::default();
        let xs = s.intersect(r).expect("must have intersections");
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let ray = Ray::new(point(0, 0, -5), vector(0, 0, 1));
        let s = Sphere::default();

        let xs = s.intersect(ray).expect("Must intersect");

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].obj, s);
        assert_eq!(xs[1].obj, s);
    }

    #[test]
    fn a_spheres_default_transform() {
        let s = Sphere::default();

        assert_eq!(BaseMatrix::identity(), s.transform);
    }

    #[test]
    fn changing_a_spheres_transformation() {
        let mut s = Sphere::default();

        let m = BaseMatrix::identity().translate(2, 3, 4);
        s.transform = m;

        assert_eq!(BaseMatrix::identity().translate(2, 3, 4), s.transform);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(point(0, 0, -5), vector(0, 0, 1));
        let mut s = Sphere::default();

        s.transform = BaseMatrix::identity().scale(2, 2, 2);
        let xs = s.intersect(r).expect("Must intersect");

        assert_eq!(2, xs.len());

        assert_eq!(3.0, xs[0].t);
        assert_eq!(7.0, xs[1].t);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(point(0, 0, -5), vector(0, 0, 1));
        let mut s = Sphere::default();

        s.transform = BaseMatrix::identity().translate(5, 0, 0);
        let xs = s.intersect(r);

        assert_eq!(None, xs);
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_x_axis() {
        let s = Sphere::default();

        let normal = s.normal_at(point(1, 0, 0));

        assert_eq!(vector(1, 0, 0), normal);
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_y_axis() {
        let s = Sphere::default();

        let normal = s.normal_at(point(0, 1, 0));

        assert_eq!(vector(0, 1, 0), normal);
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_z_axis() {
        let s = Sphere::default();

        let normal = s.normal_at(point(0, 0, 1));

        assert_eq!(vector(0, 0, 1), normal);
    }

    #[test]
    fn normal_on_a_sphere_at_a_nonaxial_point() {
        let n = 3f64.sqrt() / 3.0;
        let s = Sphere::default();

        let normal = s.normal_at(point(n, n, n));

        assert_eq!(vector(n, n, n), normal);
    }

    #[test]
    fn normal_is_a_normalized_vector() {
        let n = 3f64.sqrt() / 3.0;
        let s = Sphere::default();

        let normal = s.normal_at(point(n, n, n));

        assert_eq!(normal, normal.normalize());
    }

    #[test]
    fn computing_normal_on_translated_sphere() {
        let mut s = Sphere::default();
        s.transform = BaseMatrix::identity().translate(0, 1, 0);

        let n = s.normal_at(point(0, 1.70711, -FRAC_1_SQRT_2));

        assert_eq!(vector(0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2), n);
    }

    #[test]
    fn computing_normal_on_transformed_sphere() {
        let mut s = Sphere::default();
        let m = BaseMatrix::identity().scale(1, 0.5, 1) * BaseMatrix::identity().rotate_z(PI / 5.0);
        s.transform = m;

        let n = s.normal_at(point(0, SQRT_2 / 2.0, -SQRT_2 / 2.0));

        assert_eq!(vector(0, 0.97014, -0.24254), n);
    }

    #[test]
    fn sphere_has_default_material() {
        let s = Sphere::default();

        let m = s.material;

        assert_eq!(Material::default(), m);
    }

    #[test]
    fn sphere_may_be_assigned_a_material() {
        let mut s = Sphere::default();
        let mut m = Material::default();
        m.ambient = 1.0;

        s.material = m;

        assert_eq!(m, s.material);
    }
}
