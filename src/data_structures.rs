pub fn structures() {
  struct Point {
    x: f64,
    y: f64
  }

  struct Line {
    start: Point,
    end:   Point
  }

  let p = Point{ x: 3.0, y: 4.0 };
  println!("Point p is at ({},{})", p.x, p.y);

  let p2 = Point{ x: 5.0, y: 10.0 };
  println!("Point p2 is at ({},{})", p.x, p.y);

  let my_line = Line{ start: p, end: p2 };
  println!("Our line is from ({},{}), to ({},{})", my_line.start.x, my_line.start.y, my_line.end.x, my_line.end.y);
}

pub fn enumerations() {
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
//    _ => println!("some other colour"),
    Colour::Blue  => println!("b"), // Rust forces a match on an Enum to be exhaustive.
    Colour::RgbColour(0,0,0) => println!("black"),
    Colour::CmykColour { cyan: _, magenta: _, yellow: _, black: 255 } => println!("black"),
    Colour::RgbColour(r,g,b) => println!("rgb({},{},{})", r,g,b),
    _ => ()
  }
}

union IntOrFloat {
  i: i32,
  f: f32
}

fn process_value(iof: IntOrFloat) {
  unsafe {
    match iof {
      IntOrFloat { i: 42 } => println!("meaning of life"),
      IntOrFloat { f } => println!("f32 = {}", f)
    }
  }
}

pub fn unions() {
  let mut iof = IntOrFloat{ i: 10 };
  println!("iof value is {}", unsafe { iof.i });

  iof.i = 42;

  process_value(iof);
  process_value(IntOrFloat { f: 1.23 });
}