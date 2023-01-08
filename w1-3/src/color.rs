use crate::vec3::Color;

pub fn write_color(pixel_color: &Color, samples_per_pixel: i32) {
  let scale = 1.0 / (samples_per_pixel as f64);

  let r = (scale * pixel_color.x).sqrt();
  let g = (scale * pixel_color.y).sqrt();
  let b = (scale * pixel_color.z).sqrt();

  println!(
    "{} {} {}",
    (256.0 * r.clamp(0.0, 0.999)) as i32,
    (256.0 * g.clamp(0.0, 0.999)) as i32,
    (256.0 * b.clamp(0.0, 0.999)) as i32
  );
}
