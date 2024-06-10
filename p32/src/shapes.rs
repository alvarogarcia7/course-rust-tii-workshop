// Create shapes module
// ● Create a Shape trait with associated constant NAME and the following methods:
// – perimeter: compute shape’s perimeter
// – area: compute shape’s area
// – scale: accepts factor as `f32` and scales the shape
// – area_to_perimiter: calculates area-to-perimiter ratio (should be equal to 0 for Point)
// – biggest_area: accepts two shapes (with potentially different types) and returns reference to a shape with the biggest area
// – print_properties: prints name, area, and perimeter of the shape

use std::f64::consts::PI;

pub enum Either<Left, Right> {
    Left(Left),
    Right(Right),
}

pub trait Shape {
    const NAME: &'static str;

    fn perimeter(&self) -> u64;
    fn area(&self) -> f64;
    fn scale(&mut self, ratio: f32);
    #[allow(dead_code)]
    fn area_to_perimeter(&self) -> f64;
    #[allow(dead_code)]
    // Dynamic dispatch is not recommended
    // Enum (= tagged union) of result
    // fn biggest_area<'a, T: Shape, T2: Shape>(my_shape: &'a T2, another: &'a T) -> &'a T { -> Note: forces Shape::biggest_area -> inconvenient
    // fn biggest_area<'a, T: Shape, T2: Shape>(self: &Self, another: &'a T) -> &'a T { -> Note: don't use this in practice. Create a method for Box<Self>, for dot-syntax
    fn biggest_area<'a, 'b, T: Shape>(&'a self, another: &'b T) -> Either<&'a Self, &'b T> {
        if self.area() > another.area() {
            Either::Left(self)
        } else {
            Either::Right(another)
        }
    }
    #[allow(dead_code)]
    fn print_properties(&self);
}

struct Square {
    sides: u64,
}

impl Shape for Square {
    const NAME: &'static str = "Square";

    fn perimeter(&self) -> u64 {
        4 * self.sides
    }

    fn area(&self) -> f64 {
        (self.sides * self.sides) as f64
    }

    fn scale(&mut self, ratio: f32) {
        self.sides = (self.sides as f32 * ratio) as u64
    }

    fn area_to_perimeter(&self) -> f64 {
        self.area() / self.perimeter() as f64
    }

    fn print_properties(&self) {
        todo!()
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

impl Shape for Rectangle {
    const NAME: &'static str = "Square";

    fn perimeter(&self) -> u64 {
        2 * self.height + 2 * self.width
    }

    fn area(&self) -> f64 {
        self.width as f64 * self.height as f64
    }

    fn scale(&mut self, ratio: f32) {
        self.width = (ratio * self.width as f32) as u64;
        self.height = (ratio * self.height as f32) as u64;
    }

    fn area_to_perimeter(&self) -> f64 {
        (self.area() as u64 / self.perimeter()) as f64
    }

    fn print_properties(&self) {
        todo!()
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

impl Shape for Circle {
    const NAME: &'static str = "Circle";

    fn perimeter(&self) -> u64 {
        todo!()
    }

    fn area(&self) -> f64 {
        self.radius
            .checked_mul(self.radius)
            .map(|x| (x as f64) * (PI))
            .unwrap()
    }

    fn scale(&mut self, ratio: f32) {
        self.radius = (self.radius as f32 * ratio) as u64
    }

    fn area_to_perimeter(&self) -> f64 {
        todo!()
    }

    fn print_properties(&self) {
        todo!()
    }
}

impl Circle {
    pub fn new(radius: u64) -> Self {
        Self { radius }
    }
}

#[cfg(test)]
mod tests_square {
    use super::*;

    #[test]
    pub fn area() {
        let shape = Square::new(8);

        let actual = shape.area();

        assert_eq!(actual, 8f64 * 8f64)
    }

    #[test]
    pub fn perimeter() {
        let shape = Square::new(8);

        let actual = shape.perimeter();

        assert_eq!(actual, 4 * 8)
    }

    #[test]
    pub fn scale() {
        let mut shape = Square::new(4);

        shape.scale(5f32);

        assert_eq!(shape.perimeter(), 4 * (4 * 5))
    }
}

#[cfg(test)]
mod tests_rectangle {
    use super::*;

    #[test]
    pub fn perimeter() {
        let shape = Rectangle::new(8, 6);

        let actual = shape.perimeter();

        assert_eq!(actual, 2 * 8 + 2 * 6)
    }

    #[test]
    pub fn area() {
        let shape = Rectangle::new(8, 6);

        let actual = shape.area();

        assert_eq!(actual, 8f64 * 6f64)
    }

    #[test]
    pub fn scale() {
        let mut shape = Rectangle::new(4, 8);

        shape.scale(5f32);

        assert_eq!(shape.perimeter(), 5 * (2 * 4 + 2 * 8))
    }
}

#[cfg(test)]
mod tests_circle {
    use super::*;

    #[test]
    pub fn area() {
        let shape = Circle::new(3);

        let actual = shape.area();

        assert_eq!(actual, PI * 3f64 * 3f64);
    }

    #[test]
    pub fn scale() {
        let mut shape = Circle::new(4);

        shape.scale(5f32);

        assert_eq!(shape.area(), PI * (5u64 * 4u64).pow(2) as f64)
    }
}
