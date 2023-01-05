use weekend::ray::Ray;
use weekend::vec3::Vec3;
use weekend::vec3::write_color;
use weekend::vec3::Point3;
use weekend::hittable::HitRecord;
use weekend::hittable::Hittable;
use weekend::hittable_list::HittableList;
use weekend::sphere::Sphere;

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
  let oc = r.origin - center;
  let a = r.direction.length_squared();
  let half_b = oc.dot(&r.direction);
  let c = oc.length_squared() - radius * radius;
  let discriminant = half_b*half_b - a*c;
  
  if discriminant < 0.0 {
    -1.0
  } else {
    (-half_b - discriminant.sqrt()) / a
  }
}

fn ray_color(r: &Ray, world: &dyn Hittable) -> Vec3 {
  match world.hit(r, 0.0, f64::INFINITY) {
    None => {}
    Some(rec) => {
      return (rec.normal + Vec3 { x: 1.0, y: 1.0, z: 1.0 }) * 0.5;
    }
  }
  let unit_direction = r.direction.unit_vector();
  let t = 0.5 * (unit_direction.y + 1.0);
  Vec3 { x: 1.0, y: 1.0, z: 1.0 } * (1.0 - t) + Vec3 { x: 0.5, y: 0.7, z: 1.0 } * t
}

fn main() {
  let aspect_ratio = 16.0 / 9.0;
  let image_width = 384;
  let image_height = ((image_width as f64) / aspect_ratio) as i64;

  println!("P3");
  println!("{} {}", image_width, image_height);
  println!("255");

  let viewport_heght = 2.0;
  let viewport_width = aspect_ratio * viewport_heght;
  let focal_length = 1.0;

  let origin = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
  let horizontal = Vec3 { x: viewport_width, y: 0.0, z: 0.0};
  let vertical = Vec3 { x: 0.0, y: viewport_heght, z: 0.0 };
  let lower_left_corner =
    origin - horizontal/2.0 - vertical/2.0 - Vec3 { x: 0.0, y: 0.0, z: focal_length };

  let mut world = HittableList::new();
  world.add(&Sphere {
    center: Vec3 { x: 0.0, y: 0.0, z: -1.0 },
    radius: 0.5
  });
  world.add(&Sphere {
    center: Vec3 { x: 0.0, y: -100.5, z: -1.0 },
    radius: 100.0
  });
  for j in (0 .. image_height).rev() {
    eprint!("\rScanlines remaining: {} ", j);
    for i in 0 .. image_width {
      let u = (i as f64) / (image_width) as f64;
      let v = (j as f64) / (image_height) as f64;
      let r = Ray {
        origin: origin,
        direction: lower_left_corner + horizontal*u + vertical*v - origin
      };
      let pixel_color = ray_color(&r, &world);
      write_color(&pixel_color);    
    }
  }
  eprintln!("\nDone.");
}
