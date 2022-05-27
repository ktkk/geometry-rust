use crate::geometry::Point;
use crate::geometry::Area;

#[derive(Debug)]
pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
}

impl Triangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Triangle {
            p1: Point { x, y },
            p2: Point { x: x + width, y },
            p3: Point { x: (x + width) / 2.0, y: height }
        }
    }
}

impl Area for Triangle {
    fn area(&self) -> f32 {
        let Triangle {
            p1: Point { x: x1, y: y1 },
            p2: Point { x: x2, y: y2 },
            p3: Point { x: x3, y: y3 },
        } = self;

        0.5 * ((x1 - x3) * (y2 - y1) - (x1 - x2) * (y3 - y1)).abs()
    }
}
