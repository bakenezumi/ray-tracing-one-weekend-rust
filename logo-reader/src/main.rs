use blend::Blend;

/// Prints the name and position of every object
fn main() {
    let blend = Blend::from_path("rust_logo.blend");

    for obj in blend.get_by_code(*b"OB") {
        let loc = obj.get_f32_vec("loc");
        let name = obj.get("id").get_string("name");

        println!("\"{}\" at {:?}", name, loc);
    }
}
