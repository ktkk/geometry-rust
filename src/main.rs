mod geometry;
use geometry::Area;

fn main() {
    let r = geometry::Rectangle::new(0.0, 0.0, 5.0, 5.0);
    let t = geometry::Triangle::new(0.0, 0.0, 5.0, 5.0);

    println!("Rectangle {:?}, area: {}", r, r.area());
    println!("Triangle {:?}, area: {}", t, t.area());
}
