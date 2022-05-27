use crate::geometry::Point;
use crate::geometry::Area;

#[derive(Debug)]
pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Rectangle {
            top_left: Point { x, y },
            bottom_right: Point {
                x: x + width,
                y: y + height,
            }
        }
    }
}

impl Area for Rectangle {
    fn area(&self) -> f32 {
        let Rectangle {
            top_left: Point { x: x1, y: y1 },
            bottom_right: Point { x: x2, y: y2 },
        } = self;

        ((x2 - x1) * (y2 - y1)).abs()
    }
}
