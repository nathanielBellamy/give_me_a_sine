/*
    format.rs
*/

pub fn divider(count: u8) {
    let mut i: u8 = 0;
    while i < count {
        println!("====================");
        i += 1;
    }
}

pub fn empty_line(count: u8) {
    let mut i: u8 = 0;
    while i < count {
        println!("");
        i += 1;
    }
}
