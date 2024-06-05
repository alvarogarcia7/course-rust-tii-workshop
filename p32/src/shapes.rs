// Create shapes module
// ● Create a Shape trait with associated constant NAME and the following methods:
// – perimeter: compute shape’s perimeter
// – area: compute shape’s area
// – scale: accepts factor as `f32` and scales the shape
// – area_to_perimiter: calculates area-to-perimiter ratio (should be equal to 0 for Point)
// – biggest_area: accepts two shapes (with potentially different types) and returns reference to a shape with the biggest area
// – print_properties: prints name, area, and perimeter of the shape

trait Shape {
    const NAME: &'static str;
    fn perimeter(&self) -> u64;
}

struct Square {
    sides: u64,
}

impl Shape for Square {
    const NAME: &'static str = "Square";

    fn perimeter(&self) -> u64 {
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

impl Shape for Rectangle {
    const NAME: &'static str = "Square";

    fn perimeter(&self) -> u64 {
        self.width * self.height
    }
}

impl Rectangle {
    pub fn new(width: u64, height: u64) -> Self {
        Self { width, height }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn calculate_perimeter_for_square() {
        let square = Square::new(8);

        let actual = square.perimeter();

        assert_eq!(actual, 8 * 8)
    }

    #[test]
    pub fn calculate_perimeter_for_rectangle() {
        let shape = Rectangle::new(8, 6);

        let actual = shape.perimeter();

        assert_eq!(actual, 8 * 6)
    }
}
