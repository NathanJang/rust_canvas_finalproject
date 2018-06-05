//! Contains implementation of the Pixel and Canvas structs.
//!
//! As a general idea, the Canvas struct contains a vector of pixels, which each have an associated id and color.

extern crate rgb;
use self::rgb::*;
extern crate json;
use self::json::*;

/// The Pixel struct represents a single pixel (square) on the canvas.
#[derive(Debug, PartialEq)]
pub struct Pixel {
    /// The id of the pixel. Useful for updating specific pixels in both the server and client side.
    pub id : usize,

    /// A `RGB8` object of a simple RGB container.
    pub color : RGB8,
}

impl Pixel {
    /// Default Constructor
    pub fn new(id: usize) -> Self {
        Pixel {
            id,
            color: RGB8::new(0, 0, 0), // default color as black
        }
    }

    /// Changes the color of the pixel.
    pub fn change_color(&mut self, newcolor: RGB8) {
        self.color = newcolor;
    }

    /// Creates a Pixel object from json.
    /// The format must look something like:
    /// ```
    /// {
    ///     "id": _,
    ///     "color": {
    ///         "r": _,
    ///         "g": _,
    ///         "b": _
    ///     }
    /// }
    /// ```
    pub fn from_json(json: &JsonValue) -> Option<Self> {
        let id = json["id"].as_usize();
        if id.is_none(){
            return None;
        }
        let color = &json["color"];
        let r = color["r"].as_u8();
        let g = color["g"].as_u8();
        let b = color["b"].as_u8();
        if r.is_none() || g.is_none() || b.is_none() {
            return None;
        }
        Some(Pixel {
            id: id.unwrap(),
            color: RGB8::new(r.unwrap(), g.unwrap(), b.unwrap()),
        })
    }

    /// Returns a representation of the Pixel object in JSON.
    pub fn jsonfy(&self) -> JsonValue {
        object!{
            "id" => self.id,
            "color" => object!{
                "r"  => self.color.r,
                "g"  => self.color.g,
                "b"  => self.color.b,
            },
        }
    }

    /// Returns a representation of the Pixel object as a JSON string.
    pub fn stringify(&self) -> String {
        self.jsonfy().dump()
    }
}


/// Canvas struct implements the server side's implementation of the canvas.
/// It keeps track of the width, height, pixels, and the pixel_size to be used to draw the canvas on the client-side.
#[derive(Debug)]
pub struct Canvas {
    /// Width of the canvas as the number of pixels.
    width: usize,
    /// Height of the canvas as the number of pixels.
    height: usize,
    /// Size of a pixel when drawn on the client side.
    pixel_size: usize,
    /// Vector of pixels.
    pixels : Vec<Pixel>
}

// REPLY CONSTANTS
const REPLY_ENTIRE_BOARD :&str = "REPLY_ENTIRE_BOARD";

impl Canvas {
    /// Default constructor.
    pub fn new(width: usize, height: usize, pixel_size: usize) -> Self {
        // Default Constructor
        let length = width * height;
        let mut pixels = Vec::with_capacity(length);
        for id in 0..length {
            pixels.push(Pixel::new(id));
        }
        Canvas { width, height, pixel_size, pixels }
    }

    /// Updates a pixel on the canvas to the given pixel.
    pub fn update_pixel(&mut self, pixel: Pixel) {
        // Given a new pixel update, update the canvas
        let id = pixel.id;
        if id >= self.pixels.len(){
            // Error handling or log
            eprint!("Pixel id out of bound: {} >= {}", id, self.pixels.len());
            return;
        }
        self.pixels[id] = pixel;
    }

    /// Returns the representation of the Canvas as a JSON string.
    pub fn stringify(&self) -> String {
        let mut pixels_json = JsonValue::new_array();
        for p in &self.pixels {
            pixels_json.push(p.jsonfy()).expect("Error in creating json file");
        }

        let json_text = object! {
            "title"     => REPLY_ENTIRE_BOARD,
            "width"     => self.width,
            "height"    => self.height,
            "pixelSize" => self.pixel_size,
            "pixels"    => pixels_json
        };

        json_text.dump()
    }
}

impl From<Pixel> for json::JsonValue {
    /// Convenience initializer that serializes a pixel to a JSON.
    fn from(pixel: Pixel) -> JsonValue {
        pixel.jsonfy()
    }
}


#[cfg(test)]
mod test_canvas {
    use super::*;

    #[test]
    fn test_change_color() {
        let mut pixel = Pixel::new(5);
        pixel.change_color(RGB8::new(5, 6, 7));
        assert_eq!(pixel.id, 5);
        assert_eq!((pixel.color.r, pixel.color.g, pixel.color.b), (5, 6, 7));
    }

    #[test]
    fn test_pixel_stringify_0() {
        let pixel = Pixel::new(1);
        let expected = r#"{"id":1,"color":{"r":0,"g":0,"b":0}}"#;
        assert_eq!(pixel.stringify(), expected);
    }

    #[test]
    fn test_pixel_stringify_1() {
        let mut pixel = Pixel::new(5);
        pixel.change_color(RGB8::new(5, 6, 7));
        let expected = r#"{"id":5,"color":{"r":5,"g":6,"b":7}}"#;
        assert_eq!(pixel.stringify(), expected);
    }

    #[test]
    fn test_from_json_ok() {
        let mut pixel = Pixel::new(5);
        pixel.change_color(RGB8::new(5, 6, 7));
        let input = json::parse(&pixel.stringify()).unwrap();
        let parsed_pixel = Pixel::from_json(&input).unwrap();
        assert_eq!(parsed_pixel, pixel);
    }

    #[test]
    fn test_from_json_err() {
        let json = object!{
            "rust" => "is pretty cool"
        };
        assert!(Pixel::from_json(&json).is_none());
    }

    #[test]
    fn test_canvas_stringify() {
        fn pixel_json(id: usize) -> JsonValue {
            object!{
                "id" => id,
                "color" => object!{
                    "r" => 0,
                    "g" => 0,
                    "b" => 0,
                },
            }
        }

        let expected = json::stringify(object!{
            "title" => REPLY_ENTIRE_BOARD,
            "width" => 2,
            "height" => 2,
            "pixelSize" => 4,
            "pixels" => array![
                pixel_json(0),
                pixel_json(1),
                pixel_json(2),
                pixel_json(3)
            ],
        });
        let canvas = Canvas::new(2, 2, 4);
        assert_eq!(canvas.stringify(), expected);
    }
}
