use crate::vec3::Color;

pub fn write_color(pixel_color: &Color, samples_per_pixel: i64) {
  let scale = 1.0 / (samples_per_pixel as f64);

  let r = pixel_color.x * scale;
  let g = pixel_color.y * scale;
  let b = pixel_color.z * scale;

  println!(
    "{} {} {}",
    (256.0 * r.clamp(0.0, 0.999)) as i64,
    (256.0 * g.clamp(0.0, 0.999)) as i64,
    (256.0 * b.clamp(0.0, 0.999)) as i64
  );
}
