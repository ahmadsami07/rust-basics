#[cfg(test)]
use crate::rational::Rational;

#[test]
fn rational_test() {
    let mut r = Rational::new(6, 8);
    assert_eq!(format!("{:?}", r), "Rational { n: 6, d: 8 }");
    r.reduce();
    assert_eq!(format!("{:?}", r), "Rational { n: 3, d: 4 }");
    let four1 = Rational::from(4);
    let four2 = Rational::new(4, 1);
    assert_eq!(four1, four2);

    let rdisplay1 = Rational::new(5, 2);
    assert_eq!(format!("{}", rdisplay1), "5 / 2");

    let rdisplay2 = Rational::new(9, 2);
    assert_eq!(format!("{}", rdisplay2), "9 / 2");

    let from_float_test1 = 2.5;
    let r2: Rational = Rational::new(5, 2);
    let from_float_test2 = f64::from(r2);
    assert_eq!(from_float_test1, from_float_test2);

    let from_float_test3 = 4.5;
    let r2: Rational = Rational::new(9, 2);
    let from_float_test4 = f64::from(r2);
    assert_eq!(from_float_test3, from_float_test4);

    let r3: Rational = Rational::new(5, 2);
    let from_float_test5: f64 = r3.into();
    assert_eq!(from_float_test1, from_float_test5);
}
