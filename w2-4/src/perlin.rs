use rand::prelude::ThreadRng;
use rand::Rng;
use crate::vec3::Point3;

#[derive(Clone)]
pub struct Perlin {
    random_float: [f64; PERLIN_POINT_COUNT],
    perm_x: [i32; PERLIN_POINT_COUNT],
    perm_y: [i32; PERLIN_POINT_COUNT],
    perm_z: [i32; PERLIN_POINT_COUNT]
}

const PERLIN_POINT_COUNT: usize = 256;
impl Perlin {
    pub fn new(rng: & mut ThreadRng) -> Perlin {
        let mut random_float = [0.0; PERLIN_POINT_COUNT];
        for i in 0..PERLIN_POINT_COUNT {
            random_float[i] = rng.gen();
        }
        let perm_x = perlin_generate_perm(rng);
        let perm_y = perlin_generate_perm(rng);
        let perm_z = perlin_generate_perm(rng);
        Perlin {
            random_float: random_float,
            perm_x: perm_x,
            perm_y: perm_y,
            perm_z: perm_z
        }
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let i = ((4.0 * p.x) as i32 & 255) as usize;
        let j = ((4.0 * p.y) as i32 & 255) as usize;
        let k = ((4.0 * p.z) as i32 & 255) as usize;

        self.random_float[(self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize]
    }
}

fn perlin_generate_perm(rng: &mut ThreadRng) -> [i32; PERLIN_POINT_COUNT] {
    let mut p = [0; PERLIN_POINT_COUNT];
    for i in 0..PERLIN_POINT_COUNT {
        p[i] = i as i32;
    }
    permute(rng, &mut p, PERLIN_POINT_COUNT);
    p
}

fn permute(rng: &mut ThreadRng, p: &mut [i32], n: usize) {
    for i in (1..n).rev() {
        let target = rng.gen_range(0..=i);
        p.swap(i, target);
    }
}
