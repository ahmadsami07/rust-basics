// Dynamic dispatch is marginally slower comparitively.
// In the benchmark, make_circle_vec had an estimated mean running time of 19.011 µs,
// while for any_rectangle_zero_area the estimated mean running time was 20.353 µs.
// In comparison, any_shape_zero_area has estimated mean running time of 31.465 µs, marginally slower.
// Therefore, it seems like the apparent cost of dynamic dispatch is not a lot, with approximately
// 10 µs extra running time in this case.
// This may be caused due to dynamic dispatch resolving the functions at runtime rather than compile time,
// causing extra overhead and slightly increased running time.
pub trait Shape {
    fn area(&self) -> f64;
    fn description(&self) -> &str; // used to inspect types during testing
}

#[derive(Debug, Clone)]
pub struct Circle {
    radius: f64,
}
impl Circle {
    pub fn new(radius: f64) -> Circle {
        Circle { radius }
    }
    pub fn random() -> Circle {
        Circle {
            radius: rand::random::<f64>() + 1.0,
        }
    }
}
impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
    fn description(&self) -> &str {
        "circle"
    }
}

#[derive(Debug, Clone)]
pub struct Rectangle {
    width: f64,
    height: f64,
}
impl Rectangle {
    pub fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }
    pub fn random() -> Rectangle {
        Rectangle {
            width: rand::random::<f64>() + 1.0,
            height: rand::random::<f64>() + 1.0,
        }
    }
}
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn description(&self) -> &str {
        "rectangle"
    }
}

pub fn any_circle_zero_area(shapes: &Vec<Box<Circle>>) -> bool {
    shapes
        .iter()
        .fold(false, |any_zero, x| any_zero || x.area() == 0.0)
}
pub fn any_rectangle_zero_area(shapes: &Vec<Box<Rectangle>>) -> bool {
    shapes
        .iter()
        .fold(false, |any_zero, x| any_zero || x.area() == 0.0)
}
pub fn any_shape_zero_area(shapes: &Vec<Box<dyn Shape>>) -> bool {
    shapes
        .iter()
        .fold(false, |any_zero, x| any_zero || x.area() == 0.0)
}

// generate 2*n Circles
pub fn make_circle_vec(n: usize) -> Vec<Box<Circle>> {
    let mut circleVec: Vec<Box<Circle>> = Vec::new();
    for i in 0..2 * n {
        circleVec.push(Box::new(Circle::random()));
    }
    return circleVec;
}
// generate 2*n Rectangles
pub fn make_rectangle_vec(n: usize) -> Vec<Box<Rectangle>> {
    let mut rectVec: Vec<Box<Rectangle>> = Vec::new();
    for i in 0..2 * n {
        rectVec.push(Box::new(Rectangle::random()));
    }
    return rectVec;
}
// generate n Circles and n Rectangles
pub fn make_mixed_vec(n: usize) -> Vec<Box<dyn Shape>> {
    let mut rectVec: Vec<Box<Shape>> = Vec::new();
    for i in 0..n {
        rectVec.push(Box::new(Rectangle::random()));
    }
    for i in 0..n {
        rectVec.push(Box::new(Circle::random()));
    }
    return rectVec;
}
