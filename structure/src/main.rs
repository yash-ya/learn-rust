struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn calculate_distance(&self, other_point: &Point) -> f64 {
        let x_diff = other_point.x - self.x;
        let y_diff = other_point.y - self.y;
        let square_sum: f64 = (x_diff.pow(2) + y_diff.pow(2)) as f64;
        square_sum.sqrt()
    }
    fn compare(&self, other_point: &Point) -> bool {
        self.x == other_point.x && self.y == other_point.y
    }
}
fn main() {
    let point1 = Point { x: 10, y: 20 };

    let point2 = Point { x: 10, y: 20 };

    let distance = point1.calculate_distance(&point2);
    println!("Distance between points are {}", distance);
    println!("Two points are equal: {}", point1.compare(&point2));
}
