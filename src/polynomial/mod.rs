use super::traits::Numeric;

use num::{Complex as NumComplex, Num, Signed, Zero};
use std::{
    cmp::PartialOrd,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    ops::{Add, Mul, Neg, Sub},
};

#[derive(Clone)]
pub struct Polynomial<T> {
    degree: usize,
    coefficients: Vec<T>,
}

impl<T> Default for Polynomial<T>
where
    T: Num + Clone,
{
    /// Creates a new polynomial with all coefficients set to zero.
    /// - The degree of the polynomial is set to zero.
    /// - The coefficients are stored in a vector of length one.
    /// - The first element of the vector is the coefficient of the constant term.
    /// Note: The degree of the zero polynomial is set to zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vectra::polynomial::Polynomial;
    ///
    /// let p: Polynomial<i32> = Polynomial::default();
    ///
    /// assert_eq!(p.degree(), 0);
    /// assert_eq!(p.coefficients(), &vec![0]);
    /// ```
    fn default() -> Self {
        Self {
            degree: 0,
            coefficients: vec![T::zero(); 1],
        }
    }
}

impl<T> Polynomial<T>
where
    T: Num + Clone,
{
    /// Creates a new polynomial with all coefficients set to zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vectra::polynomial::Polynomial;
    ///
    /// let p: Polynomial<i32> = Polynomial::new();
    ///
    /// assert_eq!(p.degree(), 0);
    /// assert_eq!(p.coefficients(), &vec![0]);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T> Polynomial<T> {
    /// Creates a new polynomial from a vector of coefficients.
    /// - The degree of the polynomial is set to the length of the vector minus one.
    /// - The coefficients are stored in the vector in increasing order of degree.
    /// - The first element of the vector is the coefficient of the lowest degree term (i.e., a constant).
    /// - The last element of the vector is the coefficient of the highest degree term.
    /// Any missing coefficients are set to zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vectra::polynomial::Polynomial;
    /// let p1: Polynomial<i32> = Polynomial::from_coefficients(vec![]);
    /// let p2: Polynomial<i32> = Polynomial::from_coefficients(vec![1]);
    /// let p3: Polynomial<i32> = Polynomial::from_coefficients(vec![1, 4]);
    /// let p4: Polynomial<i32> = Polynomial::from_coefficients(vec![1, 4, 5]);
    ///
    /// assert_eq!(p1.degree(), 0);
    /// assert_eq!(p1.coefficients(), &vec![0]);
    ///
    /// assert_eq!(p2.degree(), 0);
    /// assert_eq!(p2.coefficients(), &vec![1]);
    ///
    /// assert_eq!(p3.degree(), 1);
    /// assert_eq!(p3.coefficients(), &vec![1, 4]);
    ///
    /// assert_eq!(p4.degree(), 2);
    /// assert_eq!(p4.coefficients(), &vec![1, 4, 5]);
    /// ```
    pub fn from_coefficients(coefficients: Vec<T>) -> Self
    where
        T: Num + Clone,
    {
        if coefficients.is_empty() {
            return Self::new();
        }

        let degree: usize = coefficients.len() - 1;
        Self {
            degree,
            coefficients,
        }
    }

    /// Returns the degree of the polynomial.
    /// - The degree of a polynomial is the highest power of the variable in the polynomial.
    /// - The degree of a constant polynomial is zero.
    /// Note: The degree of the zero polynomial is set to zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vectra::polynomial::Polynomial;
    ///
    /// let p: Polynomial<i32> = Polynomial::from_coefficients(vec![1, 4, 5]);
    ///
    /// assert_eq!(p.degree(), 2);
    /// ```
    pub fn degree(&self) -> usize {
        self.degree
    }

    pub fn coefficients(&self) -> &Vec<T> {
        &self.coefficients
    }
}

impl<T> Polynomial<T>
where
    T: Num + Clone,
{
    /// Returns the coefficient of the term with the given degree.
    /// If the degree is greater than the degree of the polynomial, None is returned.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vectra::polynomial::Polynomial;
    ///
    /// let p: Polynomial<i32> = Polynomial::from_coefficients(vec![1, 4, 5]);
    ///
    /// assert_eq!(p.get_coefficient(0), Some(1));
    /// assert_eq!(p.get_coefficient(1), Some(4));
    /// assert_eq!(p.get_coefficient(2), Some(5));
    /// assert_eq!(p.get_coefficient(3), None);
    /// ```
    pub fn get_coefficient(&self, degree: usize) -> Option<T> {
        if degree > self.degree {
            None
        } else {
            Some(self.coefficients[degree].clone())
        }
    }

    pub fn set_degree(&mut self, degree: usize) {
        if degree > self.degree {
            self.degree = degree;
            self.coefficients.resize(degree + 1, T::zero());
        }
    }

    pub fn set_coefficient(&mut self, degree: usize, coefficient: T) {
        self.set_degree(degree);
        self.coefficients[degree] = coefficient;
    }
}

impl<T> Add for Polynomial<T>
where
    T: Num + Clone + Default,
{
    type Output = Self;

    /// Adds two polynomials.
    /// The degree of the result is the maximum of the degrees of the two polynomials.
    /// The coefficients of the result are the sum of the coefficients of the two polynomials.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vectra::polynomial::Polynomial;
    ///
    /// let p1: Polynomial<i32> = Polynomial::from_coefficients(vec![1, 4, 5]);
    /// let p2: Polynomial<i32> = Polynomial::from_coefficients(vec![2, 8, 10, 12, 14]);
    ///
    /// let p3: Polynomial<i32> = p1 + p2;
    ///
    /// assert_eq!(p3.degree(), 4);
    /// assert_eq!(p3.coefficients(), &vec![3, 12, 15, 12, 14]);
    /// ```
    fn add(self, other: Self) -> Self {
        let mut result: Polynomial<T> = Self::new();
        result.set_degree(self.degree.max(other.degree));

        for i in 0..=result.degree {
            let c: T = self.get_coefficient(i).unwrap_or_default()
                + other.get_coefficient(i).unwrap_or_default();

            result.set_coefficient(i, c);
        }

        result
    }
}

impl<T> Sub for Polynomial<T>
where
    T: Num + Clone + Default,
{
    type Output = Self;

    /// Subtracts one polynomial from another polynomial.
    /// The degree of the result is the maximum of the degrees of the two polynomials.
    /// The coefficients of the result are the difference of the coefficients of the two polynomials.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vectra::polynomial::Polynomial;
    ///
    /// let p1: Polynomial<i32> = Polynomial::from_coefficients(vec![1, 4, 5]);
    /// let p2: Polynomial<i32> = Polynomial::from_coefficients(vec![2, 8, 10, 12, 14]);
    ///
    /// let p3: Polynomial<i32> = p1 - p2;
    ///
    /// assert_eq!(p3.degree(), 4);
    /// assert_eq!(p3.coefficients(), &vec![-1, -4, -5, -12, -14]);
    /// ```
    fn sub(self, other: Self) -> Self {
        let mut result: Polynomial<T> = Self::new();
        result.set_degree(self.degree.max(other.degree));

        for i in 0..=result.degree {
            let c: T = self.get_coefficient(i).unwrap_or_default()
                - other.get_coefficient(i).unwrap_or_default();

            result.set_coefficient(i, c);
        }

        result
    }
}

impl<T> Mul for Polynomial<T>
where
    T: Num + Clone + Default,
{
    type Output = Self;

    /// Multiplies two polynomials.
    /// The degree of the result is the sum of the degrees of the two polynomials.
    /// The coefficients of the result are the product of the coefficients of the two polynomials.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vectra::polynomial::Polynomial;
    ///
    /// let p1: Polynomial<i32> = Polynomial::from_coefficients(vec![1, 4, 5]);
    /// let p2: Polynomial<i32> = Polynomial::from_coefficients(vec![2, 8, 10, 12, 14]);
    ///
    /// let p3: Polynomial<i32> = p1 * p2;
    ///
    /// assert_eq!(p3.degree(), 6);
    /// assert_eq!(p3.coefficients(), &vec![2, 16, 52, 92, 112, 116, 70]);
    /// ```
    fn mul(self, other: Self) -> Self {
        let mut result: Polynomial<T> = Self::new();
        result.set_degree(self.degree + other.degree);

        for i in 0..=self.degree {
            for j in 0..=other.degree {
                let c: T = result.get_coefficient(i + j).unwrap_or_default()
                    + (self.get_coefficient(i).unwrap_or_default()
                        * other.get_coefficient(j).unwrap_or_default());

                result.set_coefficient(i + j, c);
            }
        }

        result
    }
}

impl<T> Debug for Polynomial<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut result: String = String::new();

        for (degree, coefficient) in self.coefficients.iter().enumerate().rev() {
            result.push_str(&format!("({}, {:?})", degree, coefficient));

            if degree > 0 {
                result.push_str(", ");
            }
        }

        write!(f, "[{}]", result)
    }
}

impl<T> Display for Polynomial<NumComplex<T>>
where
    T: Num + Clone + Debug + Display + PartialOrd + Signed + Zero,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut formatted_string: String = String::new();

        let z: T = T::zero();

        for (degree, coefficient) in self.coefficients.iter().enumerate().rev() {
            if coefficient.norm_sqr() != z {
                let mut result: String = String::new();

                if coefficient.re != z {
                    result.push_str(&format!("{}", coefficient.re));
                }

                if coefficient.im != z {
                    if coefficient.im > z {
                        if coefficient.re != z {
                            result.push('+');
                        }
                    } else {
                        result.push('-');
                    }

                    result.push_str(&format!("{}i", coefficient.im.abs()));
                }

                formatted_string.push_str(format!("+ ({})x^{} ", result, degree).as_str());
            }
        }

        write!(f, "{}", formatted_string)
    }
}

impl<T> Display for Polynomial<T>
where
    T: Numeric + Num + Clone + Display + Neg<Output = T> + PartialOrd + Zero,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut formatted_string: String = String::new();
        let mut is_first_term: bool = true;

        for (degree, coefficient) in self.coefficients.iter().enumerate().rev() {
            if coefficient != &T::zero() {
                let mut coefficient: T = coefficient.clone();
                let is_neg: bool = coefficient < T::zero();
                let sign: &str = if is_neg {
                    coefficient = -coefficient;
                    "- "
                } else {
                    "+ "
                };

                if is_first_term {
                    if is_neg {
                        formatted_string.push_str(sign);
                    }
                    is_first_term = false;
                } else {
                    formatted_string.push(' ');
                    formatted_string.push_str(sign);
                }

                let formatted: String = match degree {
                    0 => format!("{}", coefficient),
                    1 => format!("{}x", coefficient),
                    _ => format!("{}x^{}", coefficient, degree),
                };

                formatted_string.push_str(&formatted);
            }
        }

        write!(f, "{}", formatted_string)
    }
}
