use crate::math::{Vec3, dot};
use crate::rand::prelude::*;

#[inline]
fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut accum: f32 = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let if32 = i as f32;
                let jf32 = j as f32;
                let kf32 = k as f32;
                let weight_v = Vec3::new(u - if32, v - jf32, w - kf32);
                accum += (if32 * uu + (1.0 - if32) * (1.0 - uu))
                    * (jf32 * vv + (1.0 - jf32) * (1.0 - vv))
                    * (kf32 * ww + (1.0 - kf32) * (1.0 - ww))
                    * dot(&c[i][j][k], &weight_v);
            }
        }
    }
    accum
}

#[derive(Copy, Clone)]
pub struct Perlin {
    ranvec: [Vec3; 256],
    perm_x: [i32; 256],
    perm_y: [i32; 256],
    perm_z: [i32; 256],
}

impl Perlin {
    pub fn new() -> Self {
        Perlin {
            ranvec: perlin_generate(),
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
        }
    }
    pub fn noise(&self, p: &Vec3) -> f32 {
        let mut u = p.x() - p.x().floor();
        let mut v = p.y() - p.y().floor();
        let mut w = p.z() - p.z().floor();
        u = u * u * (3.0 - 2.0 * u);
        v = v * v * (3.0 - 2.0 * v);
        w = w * w * (3.0 - 2.0 * w);
        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;
        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::default(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let i_idx = ((i + di) & 255) as usize;
                    let j_idx = ((j + dj) & 255) as usize;
                    let k_idx = ((k + dk) & 255) as usize;
                    let index = self.perm_x[i_idx] ^ self.perm_y[j_idx] ^ self.perm_z[k_idx];
                    let index = index as usize;
                    c[di as usize][dj as usize][dk as usize] = self.ranvec[index];
                }
            }
        }
        perlin_interp(c, u, v, w)
    }

    pub fn turb(&self, p: &Vec3) -> f32 {
        let depth = 7;
        let mut accum = 0.0;
        let mut weight = 1.0;
        let mut tmp_p = p.clone();
        for i in 0..depth {
            accum += weight * self.noise(&tmp_p);
            weight *= 0.5;
            tmp_p *= 2.0;
        }
        accum.abs()
    }
}

fn perlin_generate() -> [Vec3; 256] {
    let mut rng = thread_rng();
    let mut p = [Vec3::default(); 256];
    for i in 0..256 {
        let x_random = 2.0 * rng.gen::<f32>() - 1.0;
        let y_random = 2.0 * rng.gen::<f32>() - 1.0;
        let z_random = 2.0 * rng.gen::<f32>() - 1.0;
        p[i] = Vec3::new(x_random, y_random, z_random).unit_vector()
    }
    p
}

fn perlin_generate_perm() -> [i32; 256] {
    let mut p: [i32; 256] = [0; 256];
    for i in 0..256 {
        p[i] = i as i32;
    }
    let mut rng = thread_rng();
    for i in (0..256).rev() {
        let target = rng.gen_range(0, i + 1);
        let tmp = p[i];
        p[i] = p[target];
        p[target] = tmp;
    }
    p
}
