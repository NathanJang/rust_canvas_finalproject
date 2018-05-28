extern crate rgb;
use self::rgb::*;

pub struct Pixel {
    pub id : usize, // id or position
    pub color : RGB8,
    pub author : String, // if we want to keep track of who modified the pixel
}

impl Pixel {
    pub fn new() -> Self {
        // Default Constructor
        unimplemented!();
    }
    pub fn change_color(&mut self, newcolor: RGB8) {
        self.color = newcolor;
    }
}

pub struct Canvas {
    pub pixels : Vec<Pixel> // 2D Vec? Hashmap<pixid, pixel>?
}

impl Canvas {
    pub fn new() -> Self {
        // Default Constructor
        Canvas { pixels: vec![] }
    }
    pub fn new_from_file() -> Self {
        // Build canvas from saved file
        unimplemented!();
    }
    pub fn update_board() {
        // Given a new pixel update, update the canvas
        unimplemented!();
    }

    #[allow(dead_code)]
    pub fn show_board() {
        // Print canvas just for testing purposes
        unimplemented!();
    }
}
