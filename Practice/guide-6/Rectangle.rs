// Exersice_4.rs
#[derive(Debug, Clone, Copy, PartialEq)]
struct Rectangular { base: f32, height: f32 }
#[derive(Debug, Clone, Copy, PartialEq)]
struct Square { side: f32 }
#[derive(Debug, Clone, Copy, PartialEq)]
struct Circle { radius: f32 }
#[derive(Debug, Clone, Copy, PartialEq)]
struct Ellipse { minor_radius: f32, major_radius: f32 }
#[derive(Debug, Clone, Copy, PartialEq)]
struct Triangle { base: f32, height: f32 }
#[derive(Debug, Clone, Copy, PartialEq)]
struct Rhombus { minor_diag: f32, major_diag: f32 }

// Exersice_5.rs
trait Surface { fn area(&self) -> f32; }
#[derive(Debug, Clone, Copy, PartialEq)]
struct Rectangular5 { base: f32, height: f32 }
#[derive(Debug, Clone, Copy, PartialEq)]
struct Square5 { side: f32 }
#[derive(Debug, Clone, Copy, PartialEq)]
struct Circle5 { radius: f32 }
#[derive(Debug, Clone, Copy, PartialEq)]
struct Ellipse5 { minor_radius: f32, major_radius: f32 }
#[derive(Debug, Clone, Copy, PartialEq)]
struct Triangle5 { base: f32, height: f32 }
#[derive(Debug, Clone, Copy, PartialEq)]
struct Rhombus5 { minor_diag: f32, major_diag: f32 }
impl Surface for Rectangular5 { fn area(&self) -> f32 { self.base * self.height } }
impl Surface for Square5 { fn area(&self) -> f32 { self.side * self.side } }
impl Surface for Circle5 { fn area(&self) -> f32 { 3.1416 * self.radius * self.radius } }
impl Surface for Ellipse5 { fn area(&self) -> f32 { 3.1416 * self.minor_radius * self.major_radius } }
impl Surface for Triangle5 { fn area(&self) -> f32 { 0.5 * self.base * self.height } }
impl Surface for Rhombus5 { fn area(&self) -> f32 { 0.5 * self.minor_diag * self.major_diag } }

// Exersice_6.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rectangle6 { x1: i32, y1: i32, x2: i32, y2: i32 }
impl Rectangle6 {
    fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self { Self { x1, y1, x2, y2 } }
}

// Exersice_7.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rectangle7 { x1: i32, y1: i32, x2: i32, y2: i32 }
impl Rectangle7 { fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self { Self { x1, y1, x2, y2 } } }
fn rectangle_area(r: &Rectangle7) -> i32 { (r.x2 - r.x1).abs() * (r.y2 - r.y1).abs() }

// Exersice_8.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rectangle8 { x1: i32, y1: i32, x2: i32, y2: i32 }
impl Rectangle8 {
    fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self { Self { x1, y1, x2, y2 } }
    fn area(&self) -> i32 { (self.x2 - self.x1).abs() * (self.y2 - self.y1).abs() }
}

// Exersice_9.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rectangle9 { x1: i32, y1: i32, x2: i32, y2: i32 }
impl Rectangle9 { fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self { Self { x1, y1, x2, y2 } } }
fn intersection_area9(a: &Rectangle9, b: &Rectangle9) -> i32 {
    let x1 = a.x1.max(b.x1);
    let y1 = a.y1.max(b.y1);
    let x2 = a.x2.min(b.x2);
    let y2 = a.y2.min(b.y2);
    let w = (x2 - x1).max(0);
    let h = (y2 - y1).max(0);
    w * h
}

// Exersice_10.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rectangle10 { x1: i32, y1: i32, x2: i32, y2: i32 }
impl Rectangle10 {
    fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self { Self { x1, y1, x2, y2 } }
    fn area(&self) -> i32 { (self.x2 - self.x1).abs() * (self.y2 - self.y1).abs() }
    fn intersection_area(&self, other: &Rectangle10) -> i32 {
        let x1 = self.x1.max(other.x1);
        let y1 = self.y1.max(other.y1);
        let x2 = self.x2.min(other.x2);
        let y2 = self.y2.min(other.y2);
        let w = (x2 - x1).max(0);
        let h = (y2 - y1).max(0);
        w * h
    }
}

fn main() {
    // main_5
    let r = Rectangular5 { base: 3.0, height: 4.0 };
    let s = Square5 { side: 5.0 };
    let c = Circle5 { radius: 2.0 };
    let e = Ellipse5 { minor_radius: 2.0, major_radius: 3.0 };
    let t = Triangle5 { base: 6.0, height: 2.0 };
    let rh = Rhombus5 { minor_diag: 3.0, major_diag: 4.0 };
    println!("{} {} {} {} {} {}", r.area(), s.area(), c.area(), e.area(), t.area(), rh.area());

    // main_7
    let r7 = Rectangle7::new(0,0,4,3);
    println!("{}", rectangle_area(&r7));

    // main_8
    let r8 = Rectangle8::new(0,0,4,3);
    println!("{}", r8.area());

    // main_9
    let r1 = Rectangle9::new(0,0,4,3);
    let r2 = Rectangle9::new(2,1,6,5);
    println!("{}", intersection_area9(&r1, &r2));

    // main_10
    let r10a = Rectangle10::new(0,0,4,3);
    let r10b = Rectangle10::new(2,1,6,5);
    println!("{}", r10a.intersection_area(&r10b));
}
