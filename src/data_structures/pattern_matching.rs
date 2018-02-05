#![allow(unused_variables)]

fn how_many(x: i32) -> &'static str {
  match x {
    0 => "no",
    1|2 => "one or two",
    12 => "a dozen",
    z @ 9...11 => "lots",
    _ if (x % 2 == 0) => "some",
    _ => "a few"
  }
}

fn integers() {
  for x in 0..13 {
    println!("{}: I have {} oranges", x, how_many(x));
  }
}

fn tuples() {
  let point = (3,7);
  match point {
    (0,0) => println!("origin"),
    (0,y) => println!("x axis, y = {}", y),
    (ref x,0) => println!("y axis, x = {}", x),
    (_,y) => println!("(?,{})", y)
  }
}

fn enums() {
  enum Colour {
    Red,
    Green,
    Blue,
    RgbColour(u8, u8, u8), // tuple
    CmykColour{
      cyan:    u8,
      magenta: u8,
      yellow:  u8,
      black:   u8
    }
  }

  let c:Colour = Colour::CmykColour { cyan: 10, magenta: 100, yellow: 50, black: 255 };

  match c {
    Colour::Red   => println!("r"),
    Colour::Green => println!("g"),
    Colour::Blue  => println!("b"),
    Colour::RgbColour(0,0,0) => println!("black"),
    Colour::CmykColour { black: 255,.. } => println!("black"), // !!! Instead of defining all the values, we can do ',..' after the values we care about.
    Colour::RgbColour(r,g,b) => println!("rgb({},{},{})", r,g,b),
    _ => ()
  }
}

pub fn pattern_matching() {
  println!("\t\t--- Integers");
  integers();

  println!("\t\t--- Tuples");
  tuples();

  println!("\t\t--- Enums");
  enums();
}