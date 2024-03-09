use rand::prelude::ThreadRng;
use rand::Rng;
use crate::vec3::{Point3, Vec3};

#[derive(Clone)]
pub struct Perlin {
    random_vec: [Vec3; PERLIN_POINT_COUNT],
    perm_x: [i32; PERLIN_POINT_COUNT],
    perm_y: [i32; PERLIN_POINT_COUNT],
    perm_z: [i32; PERLIN_POINT_COUNT]
}

const PERLIN_POINT_COUNT: usize = 256;
impl Perlin {
    pub fn new(rng: & mut ThreadRng) -> Perlin {
        let mut random_vec: [Vec3; PERLIN_POINT_COUNT] = [Vec3::zero(); PERLIN_POINT_COUNT];
        for i in 0..PERLIN_POINT_COUNT {
            random_vec[i] = Point3::random_range(rng, -1.0..1.0).unit_vector();
        }

        let perm_x = perlin_generate_perm(rng);
        let perm_y = perlin_generate_perm(rng);
        let perm_z = perlin_generate_perm(rng);
        Perlin {
            random_vec: random_vec,
            perm_x: perm_x,
            perm_y: perm_y,
            perm_z: perm_z
        }
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let mut u = p.x - p.x.floor();
        let mut v = p.y - p.y.floor();
        let mut w = p.z - p.z.floor();

        u = u*u*(3.0-2.0*u);
        v = v*v*(3.0-2.0*v);
        w = w*w*(3.0-2.0*w);

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        let mut c : [[[Vec3;2]; 2]; 2] = [
            [[Vec3::zero(), Vec3::zero()], [Vec3::zero(), Vec3::zero()]],
            [[Vec3::zero(), Vec3::zero()], [Vec3::zero(), Vec3::zero()]]
        ];

        for dk in 0..=1 {
            for dj in 0..=1 {
                for di in 0..=1 {
                    c[di as usize][dj as usize][dk as usize] = self.random_vec[
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
fn trilinear_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let uu = u*u*(3.0-2.0*u);
    let vv = v*v*(3.0-2.0*v);
    let ww = w*w*(3.0-2.0*w);
    let mut accum = 0.0;

    for i in 0..=1 {
        for j in 0..=1 {
            for k in 0..=1 {

                let i_f = i as f64;
                let j_f = j as f64;
                let k_f = k as f64;
                let weight_v = Vec3::new(u - i_f, v - j_f, w - k_f);

                accum +=
                    (i_f * uu + (1.0 - i_f) * (1.0 - uu)) *
                    (j_f * vv + (1.0 - j_f) * (1.0 - vv)) *
                    (k_f * ww + (1.0 - k_f) * (1.0 - ww)) * c[i][j][k].dot(&weight_v)
            }
        }
    }
    accum
}
