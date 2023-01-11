use rand::Rng;
use rand::rngs::ThreadRng;

use weekend::ray::Ray;
use weekend::vec3::Vec3;
use weekend::color::write_color;
use weekend::hittable::Hittable;
use weekend::hittable_list::HittableList;
use weekend::sphere::Sphere;
use weekend::sphere::Lambertian;
use weekend::camera::Camera;
use weekend::material::Metal;
use weekend::material::Dielactric;

fn ray_color(rng: &mut ThreadRng, r: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
  if depth <= 0 {
    return Vec3 {x: 0.0, y: 0.0, z: 0.0 };
  }
  match world.hit(r, 0.001, f64::INFINITY) {
    None => {}
    Some(rec) => {
      match rec.mat_ptr.scatter(rng, r, &rec) {
        None => {
          return Vec3::new(0.0, 0.0, 0.0)
        }
        Some((attenuation, scattered)) => {
          return attenuation * ray_color(rng, &scattered, world, depth-1);
        }
      }
    }
  }
  let unit_direction = r.direction.unit_vector();
  let t = 0.5 * (unit_direction.y + 1.0);
  Vec3 { x: 1.0, y: 1.0, z: 1.0 } * (1.0 - t) + Vec3 { x: 0.5, y: 0.7, z: 1.0 } * t
}

fn main() {
  let mut rng = Box::new(rand::thread_rng());

  let aspect_ratio = 16.0 / 9.0;
  let image_width = 384;
  let image_height = ((image_width as f64) / aspect_ratio) as i32;
  let samples_per_pixel = 100;
  let max_depth = 50;

  println!("P3");
  println!("{} {}", image_width, image_height);
  println!("255");

  let mut world = HittableList::new();
  let lambertian1 = Lambertian::new(Vec3::new(0.1, 0.3, 0.5));
  let sphere1 = Sphere::new (
    Vec3::new(0.0, 0.0, -1.0),
    0.5,
    & lambertian1
  );
  let lambertian2 = Lambertian::new(Vec3::new(0.8, 0.8, 0.0));
  let sphere2 = Sphere::new (
    Vec3::new(0.0, -100.5, -1.0),
    100.0,
    & lambertian2
  );
  let metal1 = Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0);
  let sphere3 = Sphere::new (
    Vec3::new(1.0, 0.0, -1.0),
    0.5,
    & metal1
  );
  let dialectric1 = Dielactric::new(1.5);
  let sphere4 = Sphere::new (
    Vec3::new(-1.0, 0.0, -1.0),
    0.5,
    & dialectric1
  );

  world.add(&sphere1);
  world.add(&sphere2);
  world.add(&sphere3);
  world.add(&sphere4);

  let cam = Camera::new();

  for j in (0 .. image_height).rev() {
    eprint!("\rScanlines remaining: {} ", j);
    for i in 0 .. image_width {
      let mut pixel_color = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
      for _ in 0 .. samples_per_pixel {
        let u = (i as f64 + rng.gen::<f64>()) / (image_width-1) as f64;
        let v = (j as f64 + rng.gen::<f64>()) / (image_height-1) as f64;
        let r = cam.get_ray(u, v);
        pixel_color = pixel_color + ray_color(&mut rng, &r, &world, max_depth);
      }
      
      write_color(&pixel_color, samples_per_pixel);    
    }
  }
  eprintln!("\nDone.");
}
