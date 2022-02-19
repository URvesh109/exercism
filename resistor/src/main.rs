use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;

#[repr(usize)]
#[derive(Debug, Copy, Clone, IntEnum, IntoEnumIterator, PartialEq)]
enum Resistor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Voilet = 7,
    Grey = 8,
    White = 9,
}

fn color_int(resistor: Resistor) -> u32 {
    resistor as u32
}

fn color_from_int(value: usize) -> String {
    match Resistor::from_int(value) {
        Ok(color) => format!("{color:?}"),
        Err(_) => format!("Not in a range")
    }
}

fn color_list() -> Vec<Resistor> {
    Resistor::into_enum_iter().collect()
}

fn main() {
    let green = Resistor::Green;
    println!("{}", color_int(green));
    println!("{}", color_from_int(5));
    println!("{:?}", color_list());
}
