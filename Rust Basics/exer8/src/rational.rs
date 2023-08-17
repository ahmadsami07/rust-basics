use std::fmt;

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

impl From<Rational> for f64 {
    fn from(r: Rational) -> f64 {
        return r.n as f64 / r.d as f64;
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{} / {}", self.n, self.d)
    }
}
