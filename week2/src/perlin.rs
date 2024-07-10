use nalgebra::{ComplexField, Vector3};
use rand::random;

use crate::rtweekend::{random_double, random_int};
use crate::vec3::vec3_rand;

const POINT_COUNT: i64 = 256;

pub struct Perlin {
    randvec:Vec<Vector3<f64>>,
    perm_x:Vec<i64>,
    perm_y:Vec<i64>,
    perm_z:Vec<i64>,
}

impl Perlin {
    pub fn new() -> Self {
        let mut randvec:Vec<Vector3<f64>> = Vec::new();
        for i in 0..POINT_COUNT {
            randvec.push(vec3_rand(-1.0,1.0));
        }
        Perlin {
            randvec:randvec,
            perm_x:Self::perlin_generate_perm(),
            perm_y:Self::perlin_generate_perm(),
            perm_z:Self::perlin_generate_perm(),
        }
    }

    pub fn noise (&self, p:&Vector3<f64>) -> f64 {
        let mut u = p.x - p.x.floor();
        let mut v = p.y - p.y.floor();
        let mut w = p.z - p.z.floor();

        let i = p.x.floor() as i64;
        let j = p.y.floor() as i64;
        let k = p.z.floor() as i64;

        let mut c:[[[Vector3<f64>;2];2];2] = [[[Vector3::zeros();2];2];2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.randvec[(self.perm_x[(i as usize +di) & 255] ^ self.perm_y[(j as usize +dj) & 255] ^ self.perm_z[(k as usize +dk) & 255]) as usize];
                }
            }
        }
        Self::perlin_interp(&c, u, v, w)
    }

    pub fn turb(&self, p:&Vector3<f64>, depth:i64) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p.clone();
        let mut weight = 1.0;

        for i in 0..depth {
            accum += weight * Self::noise(&self, &temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }

    pub fn perlin_generate_perm() -> Vec<i64> {
        let mut p:Vec<i64> = Vec::new();
        for i in 0..POINT_COUNT {
            p.push(i);
        }
        Self::permute(&mut p, POINT_COUNT);
        p
    }

    pub fn permute(p:&mut Vec<i64>, n:i64) {
        for i in 0..n-1 {
            let i = n-1-i;
            let target = random_int(0, i);
            let tmp = p[i as usize];
            p[i as usize] = p[target as usize];
            p[target as usize] = tmp;
        }
    }

    pub fn perlin_interp (c:&[[[Vector3<f64>;2];2];2], u:f64, v:f64, w:f64) -> f64{
        let uu = u*u*(3.0-2.0*u);
        let vv = v*v*(3.0-2.0*v);
        let ww = w*w*(3.0-2.0*w);
        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vector3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                            *(j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                            *(k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                            *c[i][j][k].dot(&weight_v);
                }
            }
        }
        accum
    }
 }