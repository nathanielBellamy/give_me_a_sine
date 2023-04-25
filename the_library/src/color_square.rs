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
pub const WHITE: char = '⬜';
pub const PURPLE: char = '🟪';
pub const BLUE: char = '🟦';
pub const GREEN: char = '🟩';
pub const YELLOW: char = '🟨';
pub const ORANGE: char = '🟧';
pub const RED: char = '🟥';
pub const BROWN: char = '🟫';

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
            0 => *COLOR_SQUARE.black,
            1 => *COLOR_SQUARE.white,
            2 => *COLOR_SQUARE.purple,
            3 => *COLOR_SQUARE.blue,
            4 => *COLOR_SQUARE.green,
            5 => *COLOR_SQUARE.yellow,
            6 => *COLOR_SQUARE.orange,
            7 => *COLOR_SQUARE.red,
            8 => *COLOR_SQUARE.brown,
            _ => *COLOR_SQUARE.black,
        }
    }

    pub fn to_index_char(c: char) -> char {
        match c {
         '⬛' => '0',
         '⬜' => '1',
         '🟪' => '2',
         '🟦' => '3',
         '🟩' => '4',
         '🟨' => '5',
         '🟧' => '6',
         '🟥' => '7',
         '🟫' => '8',
            _ => '0'
        }
    }
}
