
pub struct ColorSquare<'a> {
    pub black: &'a char,
    pub blue: &'a char,
    pub brown: &'a char,
    pub green: &'a char,
    pub orange: &'a char,
    pub purple: &'a char,
    pub red: &'a char,
    pub white: &'a char,
    pub yellow: &'a char,
}

pub const BLACK: char = '⬛';
pub const BLUE: char = '🟦';
pub const BROWN: char = '🟫';
pub const GREEN: char = '🟩';
pub const ORANGE: char = '🟧';
pub const PURPLE: char = '🟪';
pub const RED: char = '🟥';
pub const WHITE: char = '⬜';
pub const YELLOW: char = '🟨';

pub const COLOR_SQUARE: ColorSquare<'static> = ColorSquare {
    black: &BLACK,
    blue: &BLUE,
    brown: &BROWN,
    green: &GREEN,
    orange: &ORANGE,
    purple: &PURPLE,
    red: &RED,
    white: &WHITE,
    yellow: &YELLOW,
};

impl ColorSquare<'static> {
    pub fn by_index(index: u8) -> char {
        match index {
            0 => *COLOR_SQUARE.white,
            1 => *COLOR_SQUARE.blue,
            2 => *COLOR_SQUARE.green,
            3 => *COLOR_SQUARE.yellow,
            4 => *COLOR_SQUARE.orange,
            5 => *COLOR_SQUARE.purple,
            6 => *COLOR_SQUARE.red,
            _ => *COLOR_SQUARE.black,
        }
    }
}

