use point::Point;
use scene::{Cube, Element, Plane, Scene, Sphere};
use vector::Vector3;

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
        assert!(scene.width > scene.height);
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (scene.width as f64) / (scene.height as f64);
        let sensor_x =
            ((((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
        let sensor_y = (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment;

        Ray {
            origin: Point::zero(),
            direction: Vector3 {
                x: sensor_x,
                y: sensor_y,
                z: -1.0,
            }
            .normalize(),
        }
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;

    fn surface_normal(&self, hit_point: &Point) -> Vector3;
}

impl Intersectable for Element {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        match *self {
            Element::Sphere(ref s) => s.intersect(ray),
            Element::Plane(ref p) => p.intersect(ray),
            Element::Cube(ref c) => c.intersect(ray),
        }
    }

    fn surface_normal(&self, hit_point: &Point) -> Vector3 {
        match *self {
            Element::Sphere(ref s) => s.surface_normal(hit_point),
            Element::Plane(ref p) => p.surface_normal(hit_point),
            Element::Cube(ref c) => c.surface_normal(hit_point),
        }
    }
}
impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let l: Vector3 = self.center - ray.origin;
        let adj = l.dot(&ray.direction);
        let d2 = l.dot(&l) - (adj * adj);
        let radius2 = self.radius * self.radius;
        if d2 > radius2 {
            return None;
        }
        let thc = (radius2 - d2).sqrt();
        let t0 = adj - thc;
        let t1 = adj + thc;

        if t0 < 0.0 && t1 < 0.0 {
            return None;
        }

        let distance = if t0 < t1 { t0 } else { t1 };
        Some(distance)
    }

    fn surface_normal(&self, hit_point: &Point) -> Vector3 {
        (*hit_point - self.center).normalize()
    }
}
impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let normal = self.normal.normalize();
        let denom = normal.dot(&ray.direction);
        if denom > 1e-6 {
            let v = self.origin - ray.origin;
            let distance = v.dot(&normal) / denom;
            if distance >= 0.0 {
                return Some(distance);
            }
        }
        None
    }

    fn surface_normal(&self, _: &Point) -> Vector3 {
        -self.normal.normalize()
    }
}

impl Intersectable for Cube {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let invdir = Vector3 {
            x: 1.0 / ray.direction.x,
            y: 1.0 / ray.direction.y,
            z: 1.0 / ray.direction.z,
        };

        let mut tmin = (self.mins.x - ray.origin.x) * invdir.x;
        let mut tmax = (self.maxs.x - ray.origin.x) * invdir.x;
        let tymin = (self.mins.y - ray.origin.y) * invdir.y;
        let tymax = (self.maxs.y - ray.origin.y) * invdir.y;

        if tmin > tymax || tymin > tmax {
            return None;
        }

        if tymin > tmin {
            tmin = tymin;
        }

        if tymax < tmax {
            tmax = tymax;
        }

        let tzmin = (self.mins.z - ray.origin.z) * invdir.z;
        let tzmax = (self.maxs.z - ray.origin.z) * invdir.z;

        if tmin > tzmax || tzmin > tmax {
            return None;
        }

        if tzmin > tmin {
            tmin = tzmin;
        }

        if tzmax < tmax {
            tmax = tzmax;
        }

        let mut t = tmin;

        if t < 0.0 {
            t = tmax;
            if t < 0.0 {
                return None;
            }
        }

        return Some(t);
    }

    fn surface_normal(&self, hit_point: &Point) -> Vector3 {
        (*hit_point - self.center).normalize()
    }
}
