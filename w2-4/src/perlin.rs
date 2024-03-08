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
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        let mut c : [[[f64;2]; 2]; 2] = [[[0.0,0.0],[0.0,0.0]],[[0.0,0.0],[0.0,0.0]]];

        for dk in 0..=1 {
            for dj in 0..=1 {
                for di in 0..=1 {
                    c[di as usize][dj as usize][dk as usize] = self.random_float[
                        (self.perm_x[((i+di) & 255) as usize] ^
                            self.perm_y[((j+dj) & 255) as usize] ^
                            self.perm_z[((k+dk) & 255) as usize]
                        ) as usize
                    ];
                }
            }
        }

        trilinear_interp(c, u, v, w)
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

#[inline]
fn trilinear_interp(c: [[[f64;2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let mut accum = 0.0;
    for i in 0..=1 {
        for j in 0..=1 {
            for k in 0..=1 {
                let i_f = i as f64;
                let j_f = j as f64;
                let k_f = k as f64;
                accum +=
                    (i_f * u + (1.0 - i_f) * (1.0 - u)) *
                        (j_f * v + (1.0 - j_f) * (1.0 - v)) *
                        (k_f * w + (1.0 - k_f) * (1.0 - w)) * c[i][j][k]
            }
        }
    }
    accum
}
