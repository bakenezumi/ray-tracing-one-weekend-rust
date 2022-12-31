fn main() {
  let image_width = 256;
  let image_height = 256; 

  println!("P3");
  println!("{} {}", image_width, image_height);
  println!("255");
  for j in 0 .. image_height {
    eprint!("\rScanlines remaining: {} ", j);
    for i in 0 .. image_width {
      let r = (i as f64) / (image_width) as f64;
      let g = (j as f64) / (image_height) as f64;
      let b = 0.25;
      
      let ir = (r * 255.999) as i64;
      let ig = (g * 255.999) as i64;
      let ib = (b * 255.999) as i64;

      println!("{} {} {}", ir, ig, ib);    
    }
  }
  eprintln!("\nDone.");
}
