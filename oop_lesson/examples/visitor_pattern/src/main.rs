mod points;
use points::points_module::{MovableObjects, Moving, Point1D, Point2D};
fn main() {
    let mut p0 = Point1D::default();
    p0.to_left(&Moving::default());
    p0.to_left(&Moving::default());
    p0.to_left(&Moving::default());
    println!("point1d value after method to_left applied x3: {p0:?}");

    let mut p1 = Point2D::default();
    //same method for a visitor
    p1.to_left(&Moving::default());
    p1.to_left(&Moving::default());
    p1.apply_factor(&Moving::apply_factor(42));
    println!(
        "point2d value after method to_left applied x2, factor x42: {p1:?}"
    );
}
