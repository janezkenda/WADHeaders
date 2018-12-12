mod wad;

use wad::WAD;

fn main() {
    let buffer = wad::open_wad("DOOM.WAD");

    // File type
    for i in 0..4 {
        print!("{}", buffer.read_char(i));
    }
    println!();

    // Directory entry count
    println!("{}", buffer.read_i32(4));

    // Directory offset
    println!("{}", buffer.read_i32(8));
}
