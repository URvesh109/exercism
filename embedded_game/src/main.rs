use std::fmt::Debug;

#[allow(dead_code)]

struct Position(i16, i16);

impl Position {
    fn manhattan(&self) -> i16 {
        let Position(x, y) = self;
        x.abs() + y.abs()
    }
}

fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    let quotient = dividend / divisor;
    let remainder = dividend % divisor;
    (quotient, remainder)
}

fn evens<T: Debug>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    iter.step_by(2)
}
fn main() {
    let even_inits = evens(0_u8..10);
    let even_odds = evens(1_u8..10);
    println!("{:?}", divmod(10, 3));
    for item in even_inits {
        println!("{item}");
    }
    println!();
    for item in even_odds {
        println!("{item}");
    }

    let p1 = Position(-3, 7);
    println!("{}", p1.manhattan());
}
