use std::ops::Add;
use std::ops::Mul;

#[derive(Debug, Copy, Clone)]
pub struct Complex128 {
    x: f64,
    y: f64,
}

impl Complex128 {
    pub fn new(x: f64, y: f64) -> Complex128 {
        Complex128 { x, y }
    }

    pub fn abs(self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl Mul for Complex128 {
    type Output = Self;
    fn mul(self, z2: Self) -> Self {
        Complex128::new(self.x * z2.x - self.y * z2.y, self.x * z2.y + z2.x * self.y)
    }
}

impl Add for Complex128 {
    type Output = Self;
    fn add(self, z2: Self) -> Self {
        Complex128::new(self.x+z2.x, self.y+z2.y)
    }
}

#[cfg(test)]
mod tests {
    use super::Complex128;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_mul() {
        let z1 = Complex128::new(3.0, 4.0);
        let z2 = Complex128::new(3.0, 4.0);
        let z3 = z1 * z2;
        assert_eq!(z3.x, -7.0);
        assert_eq!(z3.y, 24.0);
    }
    #[test]
    fn test_abs() {
        let z1 = Complex128::new(3.0, 4.0);
        assert_eq!(z1.abs(), 5.0);
    }
}
