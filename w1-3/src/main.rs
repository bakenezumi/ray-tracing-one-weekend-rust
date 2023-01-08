use rand::Rng;

use weekend::ray::Ray;
use weekend::vec3::Vec3;
use weekend::color::write_color;
use weekend::hittable::Hittable;
use weekend::hittable_list::HittableList;
use weekend::sphere::Sphere;
use weekend::camera::Camera;

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
  let mut rng = rand::thread_rng();

  let aspect_ratio = 16.0 / 9.0;
  let image_width = 384;
  let image_height = ((image_width as f64) / aspect_ratio) as i64;
  let samples_per_pixel = 100;

  println!("P3");
  println!("{} {}", image_width, image_height);
  println!("255");

  let mut world = HittableList::new();
  world.add(&Sphere {
    center: Vec3 { x: 0.0, y: 0.0, z: -1.0 },
    radius: 0.5
  });
  world.add(&Sphere {
    center: Vec3 { x: 0.0, y: -100.5, z: -1.0 },
    radius: 100.0
  });

  let cam = Camera::new();

  for j in (0 .. image_height).rev() {
    eprint!("\rScanlines remaining: {} ", j);
    for i in 0 .. image_width {
      let mut pixel_color = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
      for _ in 0 .. samples_per_pixel {
        let u = (i as f64 + rng.gen::<f64>()) / (image_width-1) as f64;
        let v = (j as f64 + rng.gen::<f64>()) / (image_height-1) as f64;
        let r = cam.get_ray(u, v);
        pixel_color = pixel_color + ray_color(&r, &world);
      }
      
      write_color(&pixel_color, samples_per_pixel);    
    }
  }
  eprintln!("\nDone.");
}
