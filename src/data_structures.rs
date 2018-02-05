use std::mem;

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

pub fn options() {
  // Option<T>
  let x = 3.0;
  let y = 2.0;

  // Some(z) || None
  let result:Option<f64> =
    if y != 0.0 { Some(x/y) } else { None };

  println!("{:?}", result);

  match result {
    Some(z) => println!("{}/{}={}", x, y, z),
    None => println!("You divided by 0!")
  }

  // if let / while let
  if let Some(z) = result { println!("z = {}", z) };
}

pub fn arrays() {
//  let a:[i32;5] = [1,2,3,4,5]; // explicit type
  let mut a = [1,2,3,4,5]; // implicit type

  println!("a has {} elements, and the first is {}", a.len(), a[0]);

  a[0] = 321;
  println!("a[0] = {}", a[0]);

  println!("{:?}", a);

  if a != [1,2,3,4,5] {
    println!("Does not match [1,2,3,4,5]");
  }

  if a == [321,2,3,4,5] {
    println!("Matches [321,2,3,4,5]");
  }

//  let b = [1; 10]; // implicit [i32; 10] - 40 bytes
//  let b:[i16; 10] = [1; 10]; // explicit [i16; 10] - 20 bytes
  let b = [1u16; 10]; // implicit-ish [i16; 10] - 20 bytes
  for i in 0..b.len() {
    println!("b[{}]={}", i, b[i]);
  }
  println!("b took up {} bytes", mem::size_of_val(&b));

  let mtx:[[f32; 3]; 2] =
    [
      [1.0, 0.0, 0.0],
      [0.0, 2.0, 0.0]
    ];
  println!("{:?}", mtx);

  for i in 0..mtx.len() {
    for j in 0..mtx[i].len() {
      if i == j {
        println!("mtx[{}][{}] = {}", i, j, mtx[i][j])
      }
    }
  }


}