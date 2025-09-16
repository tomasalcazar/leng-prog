// Exercise_11.rs
use std::cmp::{Ord, Ordering};

#[derive(Debug, Clone, Copy, Eq, PartialEq)] // Los elementos que componen ese objeto usen su clone para no delegar clone a lo loco
struct Pair {
    x: i32,
    y: i32,
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.x.cmp(&other.x) {
            Ordering::Equal => self.y.cmp(&other.y),
            non_equal => non_equal,
        }
    }
}

impl PartialOrd for Pair { // Para que Pair implemente Ord, también debe implementar PartialOrd
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn max_in_slice<T: Ord>(xs: &[T]) -> Option<&T> { // Con Ord se puede usar intercambiablemente los operadores <, >, <=, >=
    xs.iter().max()
}

// Exercise_12.rs
enum ListBox {
    Cons(i32, Box<ListBox>),
    Nil,
}
use ListBox::{Cons as BCons, Nil as BNil};

// Exercise_13.rs
use std::rc::Rc;

enum ListRc {
    Cons(i32, Rc<ListRc>),
    Nil,
}
use ListRc::{Cons as RCons, Nil as RNil};

fn main() {
    // main_11
    let pairs = [
        Pair { x: 1, y: 2 },
        Pair { x: 2, y: 1 },
        Pair { x: 2, y: 2 },
        Pair { x: 0, y: 10 },
    ];
    println!("max pair = {:?}", max_in_slice(&pairs)); // -> Some(Pair { x: 2, y: 2 })

    // main_12
    let list = BCons(1, Box::new(BCons(2, Box::new(BCons(3, Box::new(BNil))))));
    let _ = list;
    println!("boxed list built");

    // main_13
    let a = Rc::new(RCons(5, Rc::new(RCons(10, Rc::new(RNil)))));
    let b = RCons(3, Rc::clone(&a));
    let c = RCons(4, Rc::clone(&a));
    let _ = (b, c);
    println!("rc lists built and share tail a");
}
