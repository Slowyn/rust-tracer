#[derive(Copy, Clone)]
pub struct Vec3(f32, f32, f32);

pub fn dot(vec1: &Vec3, vec2: &Vec3) -> f32 {
    vec1.0 * vec2.0 + vec1.1 * vec2.1 + vec1.2 * vec2.2
}

impl Vec3 {
    pub fn default() -> Self  {
        Vec3(0.0, 0.0, 0.0)
    }
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3(x, y, z)
    }
    pub fn x(&self) -> f32 { self.0 }
    pub fn y(&self) -> f32 { self.1 }
    pub fn z(&self) -> f32 { self.2 }

    pub fn r(&self) -> f32 { self.0 }
    pub fn g(&self) -> f32 { self.1 }
    pub fn b(&self) -> f32 { self.2 }

    pub fn length(&self) -> f32 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn make_unit_vector(&mut self) {
        let k = 1.0 / self.length();
        self.0 *= k; self.1 *= k; self.2 *= k;
    }

    pub fn dot(&self, vec: Vec3) -> f32 {
        self.0 * vec.0 + self.1 * vec.1 + self.2 * vec.2
    }

    pub fn cross(&self, vec: Vec3) -> Vec3 {
        Vec3::new(
            self.1 * vec.2 - self.2 * vec.1,
            -(self.0 * vec.2 - self.2 * vec.0),
            self.0 * vec.1 - self.1 * vec.0,
        )
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self {
        Vec3::new(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Vec3) -> Self {
        Vec3::new(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3::new(-self.0, -self.1, -self.2)
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Vec3) -> Self {
        Vec3::new(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl std::ops::Div for Vec3 {
    type Output = Self;
    fn div(self, rhs: Vec3) -> Self {
        Vec3::new(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Vec3 {
        Vec3::new(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Vec3 {
        Vec3::new(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(rhs.0 * self, rhs.1 * self, rhs.2 * self)
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl std::ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}

impl std::ops::DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
        self.2 /= rhs.2;
    }
}

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl std::ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl std::ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        let k = 1.0 / rhs;
        self.0 *= k;
        self.1 *= k;
        self.2 *= k;
    }
}
