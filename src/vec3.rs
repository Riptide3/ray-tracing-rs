use crate::utils;
use std::io;
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec3(pub f64, pub f64, pub f64);
pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn r(&self) -> f64 {
        self.0
    }

    pub fn g(&self) -> f64 {
        self.1
    }

    pub fn b(&self) -> f64 {
        self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.0.powi(2) + self.1.powi(2) + self.2.powi(2)
    }

    pub fn dot(&self, v: Vec3) -> f64 {
        let u = self;
        u.0 * v.0 + u.1 * v.1 + u.2 * v.2
    }

    pub fn cross(&self, v: Vec3) -> Vec3 {
        let u = self;
        Vec3(
            u.1 * v.2 - u.2 * v.1,
            u.2 * v.0 - u.0 * v.2,
            u.0 * v.1 - u.1 * v.0,
        )
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn random() -> Vec3 {
        Vec3(utils::random(), utils::random(), utils::random())
    }

    pub fn random_in(min: f64, max: f64) -> Vec3 {
        Vec3(
            utils::random_in(min, max),
            utils::random_in(min, max),
            utils::random_in(min, max),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_in(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            } else {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::unit_vector(&Vec3::random_in_unit_sphere())
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if in_unit_sphere.dot(*normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3(
                utils::random_in(-1.0, 1.0),
                utils::random_in(-1.0, 1.0),
                0.0,
            );
            if p.length_squared() >= 1.0 {
                continue;
            } else {
                return p;
            }
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.0.abs() < s && self.1.abs() < s && self.2.abs() < s
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * v.dot(n) * n
    }

    pub fn fill(v: f64) -> Vec3 {
        Vec3(v, v, v)
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-*uv).dot(*n).min(1.0);
        let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * *n;
        r_out_perp + r_out_parallel
    }
}

impl Color {
    pub fn write_color<T: io::Write>(&self, out: &mut T, samples_per_pixel: i32) -> io::Result<()> {
        let mut r = self.r();
        let mut g = self.g();
        let mut b = self.b();

        //根据样本数对颜色取平均值
        let scale = 1.0 / samples_per_pixel as f64;
        r = (scale * r).sqrt();
        g = (scale * g).sqrt();
        b = (scale * b).sqrt();

        let s = format!(
            "{} {} {}\n",
            (256.0 * utils::clamp(r, 0.0, 0.999)) as i32,
            (256.0 * utils::clamp(g, 0.0, 0.999)) as i32,
            (256.0 * utils::clamp(b, 0.0, 0.999)) as i32
        );
        out.write_all(s.as_bytes())?;
        Ok(())
    }
}

// -Vec3
impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

// Vec3 + Vec3
impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

// Vec3 - Vec3
impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

// Vec3 * Vec3
impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

// Vec3 * f64
impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

// f64 * Vec3
impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

// Vec3 / f64
impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

// Vec3 += Vec3
impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2);
    }
}

// Vec3 -= Vec3
impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2);
    }
}

// Vec3 *= Vec3
impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2);
    }
}

// Vec3 *= f64
impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self(self.0 * rhs, self.1 * rhs, self.2 * rhs);
    }
}

// Vec3 /= f64
impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn assert_approx_eq(a: f64, b: f64) {
        let n = 1.0e-6;
        assert!(
            (a - b).abs() < n,
            "assertion failed: `(left == right)` \
            (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
            a,
            b,
            n,
            (a - b).abs()
        )
    }

    #[test]
    fn test_vec3_methods() {
        let a = Vec3(0.1, 0.2, 0.3);
        assert_eq!(a.x(), 0.1);
        assert_eq!(a.y(), 0.2);
        assert_eq!(a.z(), 0.3);

        assert_eq!(a.r(), 0.1);
        assert_eq!(a.g(), 0.2);
        assert_eq!(a.b(), 0.3);

        assert_eq!(&a.length().to_string()[0..4], "0.37");
    }

    #[test]
    fn test_vec3_operators() {
        let a = Vec3(0.1, 0.2, 0.3);
        let b = Vec3(0.3, 0.2, 0.1);
        let c = a + b;
        let d = a - b;
        let e = a * b;
        let f = a * 2.0;
        let g = 2.0 * a;
        let h = a / 10.0;
        let i = -a;

        assert_approx_eq(c.x(), 0.4);
        assert_approx_eq(c.y(), 0.4);
        assert_approx_eq(c.z(), 0.4);

        assert_approx_eq(d.x(), -0.2);
        assert_approx_eq(d.y(), -0.0);
        assert_approx_eq(d.z(), 0.2);

        assert_approx_eq(e.x(), 0.03);
        assert_approx_eq(e.y(), 0.04);
        assert_approx_eq(e.z(), 0.03);

        assert_approx_eq(f.x(), 0.2);
        assert_approx_eq(f.y(), 0.4);
        assert_approx_eq(f.z(), 0.6);

        assert_approx_eq(g.x(), 0.2);
        assert_approx_eq(g.y(), 0.4);
        assert_approx_eq(g.z(), 0.6);

        assert_approx_eq(h.x(), 0.01);
        assert_approx_eq(h.y(), 0.02);
        assert_approx_eq(h.z(), 0.03);

        assert_approx_eq(i.x(), -0.1);
        assert_approx_eq(i.y(), -0.2);
        assert_approx_eq(i.z(), -0.3);
    }
}
