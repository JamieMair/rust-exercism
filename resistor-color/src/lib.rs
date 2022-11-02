use int_enum::IntEnum;
use enum_iterator::{all, Sequence};

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, IntEnum, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    return _color as u32;
}

pub fn value_to_color_string(value: u32) -> String {
    return match ResistorColor::from_int(value) {
        Ok(x) => format!("{:?}", x),
        _ => String::from("value out of range"),
    };
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect::<Vec<_>>()
}
