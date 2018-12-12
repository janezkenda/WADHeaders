use std::fs::File;
use std::io::Read;

pub trait WAD {
    fn read_char(&self, offset: usize) -> char;
    fn read_i16(&self, offset: usize) -> i16;
    fn read_u16(&self, offset: usize) -> u16;
    fn read_i32(&self, offset: usize) -> i32;
    fn read_u32(&self, offset: usize) -> u32;
}

// Methods to read variables from big-endian file.
impl WAD for Vec<u8> {
    fn read_char(&self, offset: usize) -> char {
        self[offset] as char
    }

    fn read_i16(&self, offset: usize) -> i16 {
        (self[offset + 1] as i16) << 8 | self[offset] as i16
    }

    fn read_u16(&self, offset: usize) -> u16 {
        (self[offset + 1] as u16) << 8 | self[offset] as u16
    }

    fn read_i32(&self, offset: usize) -> i32 {
        (self[offset + 3] as i32) << 24 | (self[offset + 2] as i32) << 16 | (self[offset + 1] as i32) << 8 | self[offset] as i32
    }

    fn read_u32(&self, offset: usize) -> u32 {
        (self[offset + 3] as u32) << 24 | (self[offset + 2] as u32) << 16 | (self[offset + 1] as u32) << 8 | self[offset] as u32
    }
}

// Load the WAD file into Vec<u8>
pub fn open_wad(path: &str) -> Vec<u8> {
    let mut f = File::open(path).expect("file not found");
    let mut buffer: Vec<u8> = vec![];

    f.read_to_end(&mut buffer).unwrap();

    buffer
}