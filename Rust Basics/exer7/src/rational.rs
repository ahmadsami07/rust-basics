fn gcd(a: i64, b: i64) -> i64 {
    let mut num = a;
    let mut den = b;
    while den != 0 {
        let remainder = num % den;
        num = den;
        den = remainder;
    }
    return num;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rational {
    n: i64,
    d: i64,
}

impl Rational {
    pub fn new(n: i64, d: i64) -> Rational {
        return Rational { n, d };
    }
    pub fn reduce(&mut self) -> () {
        let gcd = gcd(self.n, self.d);
        self.n = self.n / gcd;
        self.d = self.d / gcd;
    }
}

impl From<i64> for Rational {
    fn from(value: i64) -> Self {
        return Rational::new(value, 1);
    }
}
