// Create shapes module
// ● Create a Shape trait with associated constant NAME and the following methods:
// – perimeter: compute shape’s perimeter
// – area: compute shape’s area
// – scale: accepts factor as `f32` and scales the shape
// – area_to_perimiter: calculates area-to-perimiter ratio (should be equal to 0 for Point)
// – biggest_area: accepts two shapes (with potentially different types) and returns reference to a shape with the biggest area
// – print_properties: prints name, area, and perimeter of the shape

use std::f64::consts::PI;

pub trait DecimalArea {
    fn area(&self) -> f64;
}

pub trait WholeArea {
    fn area(&self) -> u64;
}

#[allow(dead_code)]
pub trait ShapeName {
    const NAME: &'static str;
}

#[allow(dead_code)]
pub trait WholeareaShape: ShapeName + WholeArea {}

struct Square {
    sides: u64,
}

impl ShapeName for Square {
    const NAME: &'static str = "Square";
}

impl WholeArea for Square {
    fn area(&self) -> u64 {
        self.sides * self.sides
    }
}

impl Square {
    pub fn new(sides: u64) -> Self {
        Self { sides }
    }
}

struct Rectangle {
    width: u64,
    height: u64,
}

impl ShapeName for Rectangle {
    const NAME: &'static str = "Square";
}

impl WholeArea for Rectangle {
    fn area(&self) -> u64 {
        self.width * self.height
    }
}

impl Rectangle {
    pub fn new(width: u64, height: u64) -> Self {
        Self { width, height }
    }
}

struct Circle {
    radius: u64,
}

impl ShapeName for Circle {
    const NAME: &'static str = "Circle";
}

impl DecimalArea for Circle {
    fn area(&self) -> f64 {
        self.radius
            .checked_mul(self.radius)
            .map(|x| (x as f64) * (PI))
            .unwrap()
    }
}

impl Circle {
    pub fn new(radius: u64) -> Self {
        Self { radius }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    pub fn calculate_area_for_square() {
        let square = Square::new(8);

        let actual = square.area();

        assert_eq!(actual, 8 * 8)
    }

    #[test]
    pub fn calculate_area_for_rectangle() {
        let shape = Rectangle::new(8, 6);

        let actual = shape.area();

        assert_eq!(actual, 8 * 6)
    }

    #[test]
    pub fn calculate_area_for_circle() {
        let shape = Circle::new(3);

        let actual = shape.area();

        assert_eq!(actual, PI * 3f64 * 3f64);
    }
}
