use std::sync::mpsc;
use std::sync::Mutex;
use rand::Rng;
use rand::rngs::ThreadRng;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::ParallelIterator;
use weekend::bvh_node::BvhNode;
use weekend::ray::Ray;
use weekend::vec3::Vec3;
use weekend::vec3::Color;
use weekend::hittable::Hittable;
use weekend::hittable_list::HittableList;
use weekend::sphere::Sphere;
use weekend::material::{DiffuseLight, Lambertian};
use weekend::camera::Camera;
use weekend::material::Metal;
use weekend::material::Dielactric;
use weekend::texture::{CheckerTexture, ImageTexture, NoiseTexture, SolidColor};
use weekend::rect::{XyRect, XzRect, YzRect};
use weekend::box_model::BoxModel;

fn ray_color(rng: &mut ThreadRng, r: &Ray, background: &Color, world: &dyn Hittable, depth: i32) -> Vec3 {
  if depth <= 0 {
    return Color::black();
  }

  match world.hit(r, 0.001, f64::INFINITY) {
    None => {
      return *background;
    },
    Some(rec) => {
      let emitted = rec.mat_ptr.emitted(rec.u, rec.v, &rec.p);
      match rec.mat_ptr.scatter(rng, r, &rec) {
        None => {
          return emitted;
        }
        Some((attenuation, scattered)) => {
          return emitted + attenuation * ray_color(rng, &scattered, background, world, depth-1);
        }
      }
    }
  }
}


fn random_scene<'a>(rng: &mut ThreadRng) -> HittableList {
  let mut world = HittableList::new();

  let checker = Box::new(CheckerTexture::new(
    Box::new(SolidColor::new(Vec3::new(0.2, 0.3, 0.1))),
    Box::new(SolidColor::new(Vec3::new(0.9, 0.9, 0.9))),
  ));

  let ground_material = Lambertian::new(checker);
  let ground = Sphere::new(
    Vec3::new(0.0, -1000.0, 0.0),
    1000.0,
    Box::new(ground_material)
  );
  world.add(Box::new(ground));

  let mut objects = HittableList::new();

  for a in -11..11 {
    for b in -11..11 {
      let choose_mat = rng.gen::<f64>();
      let center = Vec3::new((a as f64) + 0.9*rng.gen::<f64>(), 0.2, (b as f64) + 0.9*rng.gen::<f64>());

      if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
        if choose_mat < 0.8 {
          let albedo = Vec3::random(rng) * Vec3::random(rng);        
          let r = Lambertian::new(Box::new(SolidColor::new(albedo)));
          objects.add(Box::new(Sphere::new(center, 0.2, Box::new(r))));
        } else if choose_mat < 0.95 {
          let albedo = Vec3::random(rng) * Vec3::random_range(rng, 0.5..1.0);
          let fuzz = rng.gen_range(0.0..0.5);
          let r = Metal::new(albedo, fuzz);
          objects.add(Box::new(Sphere::new(center, 0.2, Box::new(r))));
        } else {
          let r = Dielactric::new(1.5);
          objects.add(Box::new(Sphere::new(center, 0.2, Box::new(r))));
        };        
      }
    }
  }

  objects.add(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Box::new(Dielactric::new(1.5)))));
  objects.add(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Box::new(Lambertian::new(Box::new(SolidColor::new(Vec3::new(0.4, 0.2, 0.1))))))));
  objects.add(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)))));

  // world.add(Box::new(objects));
  let len = objects.objects.len();
  world.add(Box::new(BvhNode::new(rng, objects, 0, len, 0.0, 0.0)));
  world
}

fn two_spheres() -> HittableList {
  let mut objects = HittableList::new();

  let checker = Box::new(CheckerTexture::new(
    Box::new(SolidColor::new(Vec3::new(0.2, 0.3, 0.1))),
    Box::new(SolidColor::new(Vec3::new(0.9, 0.9, 0.9))),
  ));

  objects.add(
    Box::new(Sphere::new(Vec3::new(0.0, -10.0, 0.0), 10.0, Box::new(Lambertian::new(checker.clone()))))
  );

  objects.add(
    Box::new(Sphere::new(Vec3::new(0.0, 10.0, 0.0), 10.0, Box::new(Lambertian::new(checker))))
  );

  objects
}

fn two_perlin_spheres(rng: &mut ThreadRng) -> HittableList {
  let mut objects = HittableList::new();

  let pertext = Box::new(NoiseTexture::new(rng, 5.0));

  objects.add(
    Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(Lambertian::new(pertext.clone()))))
  );

  objects.add(
    Box::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, Box::new(Lambertian::new(pertext))))
  );

  objects
}

fn earth() -> HittableList {
  let mut objects = HittableList::new();

  let earth_texture = Box::new(ImageTexture::new("assets/earthmap.jpg"));
  let earth_surface = Box::new(Lambertian::new(earth_texture));

  objects.add(
    Box::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 2.0, earth_surface))
  );

  objects
}

fn simple_light() -> HittableList {
  let mut objects = HittableList::new();

  let pertext = Box::new(NoiseTexture::new(&mut rand::thread_rng(), 4.0));
  objects.add(
    Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(Lambertian::new(pertext.clone()))))
  );
  objects.add(
    Box::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, Box::new(Lambertian::new(pertext.clone())))
  ));

  let difflight = Box::new(DiffuseLight::new(Box::new(SolidColor::new(Vec3::new(4.0, 4.0, 4.0)))));
  objects.add(
    Box::new(Sphere::new(Vec3::new(0.0, 7.0, 0.0), 2.0, difflight.clone()))
  );
  objects.add(
    Box::new(XyRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight))
  );

  objects
}

fn cornell_box() -> HittableList {
    let mut objects = HittableList::new();

    let red = Box::new(Lambertian::new(Box::new(SolidColor::new(Vec3::new(0.65, 0.05, 0.05)))));
    let white = Box::new(Lambertian::new(Box::new(SolidColor::new(Vec3::new(0.73, 0.73, 0.73)))));
    let green = Box::new(Lambertian::new(Box::new(SolidColor::new(Vec3::new(0.12, 0.45, 0.15)))));
    let light = Box::new(DiffuseLight::new(Box::new(SolidColor::new(Vec3::new(15.0, 15.0, 15.0)))));

    objects.add(
        Box::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green.clone()))
    );
    objects.add(
        Box::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red.clone()))
    );
    objects.add(
        Box::new(XzRect::new(213.0, 343.0, 227.0, 332.0, 554.0, light.clone()))
    );
    objects.add(
        Box::new(XzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone()))
    );
    objects.add(
        Box::new(XzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()))
    );
    objects.add(
        Box::new(XyRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()))
    );

    objects.add(Box::new(BoxModel::new(Vec3::new(130.0, 0.0, 65.0), Vec3::new(295.0, 165.0, 230.0), white.clone())));
    objects.add(Box::new(BoxModel::new(Vec3::new(265.0, 0.0, 295.0), Vec3::new(430.0, 330.0, 4600.0), white.clone())));

    objects
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

  // let aspect_ratio = 16.0 / 9.0;
  // let image_width = 400;
  let aspect_ratio = 1.0;
  let image_width = 500;

  let image_height = ((image_width as f64) / aspect_ratio) as i32;
  let samples_per_pixel = 100;
  let max_depth = 50;

  let (tx, rx) = mpsc::channel();
  let mtx = Mutex::new(tx);

  let image_generation_task = async move {
    let world = {
      // let mut rng = Box::new(rand::thread_rng());
      // random_scene(&mut rng)

      // two_spheres()

      // let mut rng = Box::new(rand::thread_rng());
      // two_perlin_spheres(&mut rng)

      // earth()

      // simple_light()

      cornell_box()
    };
  
    // let lookfrom = Vec3::new(26.0, 3.0, 6.0);
    // let lookat = Vec3::new(0.0, 2.0, 0.0);
    // let vup = Vec3::new(0.0, 1.0, 0.0);
    // let vfov = 20.0;
    // let aspect_ratio = (image_width as f64)/(image_height as f64);
    // let dist_to_focus = 10.0;
    // // let aperture = 0.1;
    // let aperture = 0.0;
    // let time0 = 0.0;
    // let time1 = 1.0;

    let lookfrom = Vec3::new(278.0, 278.0, -800.0);
    let lookat = Vec3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 40.0;
    let aspect_ratio = (image_width as f64)/(image_height as f64);
    let dist_to_focus = 10.0;
    // let aperture = 0.1;
    let aperture = 0.0;
    let time0 = 0.0;
    let time1 = 1.0;

    let cam = Camera::new(lookfrom, lookat, vup, vfov, aspect_ratio, aperture, dist_to_focus, time0, time1);
  

    (0 .. image_height).into_par_iter().for_each(|j| {
      let mut rng = Box::new(rand::thread_rng());
      for i in 0 .. image_width {
        let mut pixel_color = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
        for _ in 0 .. samples_per_pixel {
          let u = (i as f64 + rng.gen::<f64>()) / (image_width-1) as f64;
          let v = (j as f64 + rng.gen::<f64>()) / (image_height-1) as f64;
          let r = cam.get_ray(&mut rng, u, v);
          // let background = Color::new(0.0, 0.0, 0.0);
          let background = Color::new(0.0, 0.0, 0.0);
          pixel_color = pixel_color + ray_color(&mut rng, &r, &background, &world, max_depth);
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
