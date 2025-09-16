
// Exercise_1.rs
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self { Self { x, y } }

    fn distance(&self, other: Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

// Exercise_2.rs
fn are_collinear(a: Point, b: Point, c: Point) -> bool {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x) == 0.0
}

// Exercise_3.rs
fn lies_on_line(self, a: Point, b: Point) -> bool {
        (b.x - a.x) * (self.y - a.y) - (b.y - a.y) * (self.x - a.x) == 0.0
    }

fn main() {
    let a = Point::new(0.0, 0.0);
    let b = Point::new(1.0, 1.0);
    let c = Point::new(2.0, 2.0);
    let d = Point::new(2.0, 3.0);

    println!("Distance a–b: {}", a.distance(b));
    println!("Are a,b,c collinear? {}", are_collinear(a, b, c));
    println!("c on line ab? {}", c.lies_on_line(a, b));
    println!("d on line ab? {}", d.lies_on_line(a, b));
}
