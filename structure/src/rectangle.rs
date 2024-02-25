#[derive(Debug)]
struct Rectangle {
    width: usize,
    height: usize,
}
impl Rectangle {
    fn area(&self) -> usize {
        self.width * self.height
    }
}
fn play_with_rectangle() {
    let rectangle = Rectangle {
        width: 10,
        height: 20,
    };

    let area_of_rectangle = area(&rectangle);
    println!(
        "area of rectangle with width {}px and height {}px is {}sq-px",
        rectangle.width, rectangle.height, area_of_rectangle
    );
    dbg!(&rectangle);
    dbg!(rectangle.area());
    println!(
        "area of rectangle with width {}px and height {}px is {}sq-px",
        rectangle.width, rectangle.height, area_of_rectangle
    );
}

fn area(rectangle: &Rectangle) -> usize {
    rectangle.width * rectangle.height
}
