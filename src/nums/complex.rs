use num::Complex as NumComplex;
use num::Num;

pub struct Complex<T>(NumComplex<T>);

impl<T> Complex<T> {
    pub fn new(re: T, im: T) -> Self {
        Complex(NumComplex::new(re, im))
    }
}

impl<T> std::ops::Add for Complex<T>
where
    T: Num + std::ops::Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let re: T = self.0.re + rhs.0.re;
        let im: T = self.0.im + rhs.0.im;

        Complex(NumComplex::new(re, im))
    }
}
