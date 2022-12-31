mod ray;
use ray::Ray;

use ray::vec3::Vec3;
use ray::vec3::write_color;
use ray::vec3::Point3;

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> bool {
  let oc = r.origin - center;
  let a = r.direction.dot(&r.direction);
  let b = oc.dot(&r.direction) * 2.0;
  let c = oc.dot(&oc) - radius * radius;
  let discriminant = b*b - 4.0*a*c;
  discriminant > 0.0
}

fn ray_color(r: &Ray) -> Vec3 {
  if (hit_sphere(Point3{ x: 0.0, y: 0.0, z: -1.0 }, 0.5, r)) {
    return Vec3 { x: 1.0, y: 0.0, z: 0.0 }
  };
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

  for j in (0 .. image_height).rev() {
    eprint!("\rScanlines remaining: {} ", j);
    for i in 0 .. image_width {
      let u = (i as f64) / (image_width) as f64;
      let v = (j as f64) / (image_height) as f64;
      let r = Ray {
        origin: origin,
        direction: lower_left_corner + horizontal*u + vertical*v - origin
      };
      let pixel_color = ray_color(&r);
      write_color(&pixel_color);    
    }
  }
  eprintln!("\nDone.");
}
