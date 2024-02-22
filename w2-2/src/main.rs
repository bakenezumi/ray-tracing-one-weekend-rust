use std::sync::mpsc;
use std::sync::Mutex;
use rand::Rng;
use rand::rngs::ThreadRng;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::ParallelIterator;
use weekend::ray::Ray;
use weekend::vec3::Vec3;
use weekend::vec3::Color;
use weekend::hittable::Hittable;
use weekend::hittable_list::HittableList;
use weekend::sphere::{MovingSphere, Sphere};
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


fn random_scene<'a>(rng: &mut ThreadRng) -> HittableList {
  let mut world = HittableList::new();
  let ground_material = Lambertian::new(Vec3::new(0.5, 0.5, 0.5));
  let ground = Sphere::new(
    Vec3::new(0.0, -1000.0, 0.0),
    1000.0,
    Box::new(ground_material)
  );
  world.add(Box::new(ground));

  for a in -11..11 {
    for b in -11..11 {
      let choose_mat = rng.gen::<f64>();
      let center = Vec3::new((a as f64) + 0.9*rng.gen::<f64>(), 0.2, (b as f64) + 0.9*rng.gen::<f64>());

      if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
        if choose_mat < 0.8 {
          let albedo = Vec3::random(rng) * Vec3::random(rng);        
          let r = Lambertian::new(albedo);
          let center2 = center + Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
          world.add(Box::new(MovingSphere::new(center, center2, 0.0, 1.0, 0.2, Box::new(r))));
        } else if choose_mat < 0.95 {
          let albedo = Vec3::random(rng) * Vec3::random_range(rng, 0.5..1.0);
          let fuzz = rng.gen_range(0.0..0.5);
          let r = Metal::new(albedo, fuzz);
          world.add(Box::new(Sphere::new(center, 0.2, Box::new(r))));
        } else {
          let r = Dielactric::new(1.5);
          world.add(Box::new(Sphere::new(center, 0.2, Box::new(r))));
        };        
      }
    }
  }

  world.add(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Box::new(Dielactric::new(1.5)))));
  world.add(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))))));
  world.add(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)))));

  world
}

pub fn format_ppm(pixel_color: &Color, samples_per_pixel: i32) -> String {
  let scale = 1.0 / (samples_per_pixel as f64);

  let r = (scale * pixel_color.x).sqrt();
  let g = (scale * pixel_color.y).sqrt();
  let b = (scale * pixel_color.z).sqrt();

  format!(
    "{} {} {}",
    (256.0 * r.clamp(0.0, 0.999)) as i32,
    (256.0 * g.clamp(0.0, 0.999)) as i32,
    (256.0 * b.clamp(0.0, 0.999)) as i32
  )
}

#[tokio::main]
async fn main() {

  let aspect_ratio = 16.0 / 9.0;
  let image_width = 384;
  let image_height = ((image_width as f64) / aspect_ratio) as i32;
  let samples_per_pixel = 100;
  let max_depth = 50;

  let (tx, rx) = mpsc::channel();
  let mtx = Mutex::new(tx);

  let image_generation_task = async move {
    let world = {
      let mut rng = Box::new(rand::thread_rng());
      random_scene(&mut rng)
    };
  
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let aspect_ratio = (image_width as f64)/(image_height as f64);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let time0 = 0.0;
    let time1 = 1.0;
  
    let cam = Camera::new(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, dist_to_focus, time0, time1);
  

    (0 .. image_height).into_par_iter().for_each(|j| {
      let mut rng = Box::new(rand::thread_rng());
      for i in 0 .. image_width {
        let mut pixel_color = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
        for _ in 0 .. samples_per_pixel {
          let u = (i as f64 + rng.gen::<f64>()) / (image_width-1) as f64;
          let v = (j as f64 + rng.gen::<f64>()) / (image_height-1) as f64;
          let r = cam.get_ray(&mut rng, u, v);
          pixel_color = pixel_color + ray_color(&mut rng, &r, &world, max_depth);
        }
        let ppm = format_ppm(&pixel_color, samples_per_pixel);
        mtx.lock().unwrap().send(((image_height-j, i), ppm)).unwrap();
      }
    });
  };  

  tokio::spawn(image_generation_task);

  println!("P3");
  println!("{} {}", image_width, image_height);
  println!("255");

  let mut list = Vec::new();
  let mut counter = 0;
  let num_of_pixcels = image_height * image_width;
  for v in rx.iter() {
    list.push(v);
    counter = counter + 1;
    eprint!("\r{} %", counter * 100 / num_of_pixcels);
    if num_of_pixcels == list.len().try_into().unwrap() {
      break;
    }
  }
  list.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
  for (_, item) in list {
    println!("{}", item);
  }

  eprintln!("\nDone.");
}
