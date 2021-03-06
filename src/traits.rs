use std;

pub fn traits() {
  trait Animal {
    fn create(name: &'static str) -> Self;

    fn name(&self) -> &'static str;

    fn talk(&self) {
      println!("{} cannot talk", self.name());
    }
  }

  struct Human {
    name: &'static str
  }

  impl Animal for Human {
    fn create(name: &'static str) -> Human {
      Human { name: name }
    }

    fn name(&self) -> &'static str {
      self.name
    }

    fn talk(&self) {
      println!("{} says 'Hello!'", self.name())
    }
  }

  struct Cat {
    name: &'static str
  }

  impl Animal for Cat {
    fn create(name: &'static str) -> Cat {
      Cat { name: name }
    }

    fn name(&self) -> &'static str {
      self.name
    }

    fn talk(&self) {
      println!("{} says 'Meow!'", self.name())
    }
  }

//  let matt = Human { name: "Matt" };
//  let matt = Human::create("Matt");
  let matt:Human = Animal::create("Matt");
  matt.talk();

//  let harry = Cat { name: "Harry" };
  let harry = Cat::create("Harry");
  harry.talk();

  // Add traits to an object you haven't made yourself
  trait Summable<T> {
    fn sum(&self) -> T;
  }

  impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
//      self.iter().fold(0, |sum, val| sum + val) // Functional
      self.iter().sum() // Newer Rust allows sum() on Iterator
    }
  }

  let a = vec![1, 2, 3];
  println!("sum = {}", a.sum());
}

pub fn operator_overloading() {
  #[derive(Debug)]
  struct Point {
    x: f64,
    y: f64
  }

  use std::ops::Add;

  impl Add for Point {
    type Output = Point;

    fn add(self, other:Point) -> Point {
      Point { x: self.x + other.x, y: self.y + other.y }
    }
  }

  let p = Point { x: 1.0, y: 2.0 };
  let p2 = Point { x: 3.0, y: 4.0 };
  let p3 = p + p2;
  println!("{:?}", p3);
}

trait Printable {
  fn format(&self) -> String;
}

impl Printable for i32 {
  fn format(&self) -> String {
    format!("i32: {}", *self)
  }
}

impl Printable for String {
  fn format(&self) -> String {
    format!("String: {}", *self)
  }
}

// Static dispatch
fn print_it<T: Printable>(z: T) {
  println!("{}", z.format());
}

/*
At compilation time, we get 'monomorphisation' -> the above print_it function gets compiled into
discreet method implementations for each type of Printable that uses it.

For example, the two uses below, actually create:
fn print_it(z: i32) {
  println!("{}", format!("i32: {}", *self));
}
fn print_it(z: String) {
  println!("{}", format!("String: {}", *self));
}

These methods are then faster, and more efficient, to run as the calls to `print_it` below are
replaced with references to the specific print_it for that type.
*/

pub fn static_dispatch() {
  let a = 123;
  let b = "hello".to_string();

//  println!("{}", a.format());
//  println!("{}", b.format());

  print_it(a);
  print_it(b);
}

// Dynamic dispatch
fn print_it_too(z: &Printable) {
  println!("{}", z.format());
}

pub fn dynamic_dispatch() {
  let a = 123;
  let b = "hello".to_string();

  /*
  Dynamic dispatched uses Type Erasure - when passing a & b to print_it_too, you're passing a
  pointer that just says yes it's a Printable, or no, it's not.
  */

  print_it_too(&a);
  print_it_too(&b);

  /*
  Dynamic dispatch means that at run-time, print_it_too has to work out which implementation block's
  format function should be called. This cannot be optimised at compile time and as such puts a
  higher load on the application at run time.
  */
}

pub fn why_dynamic_dispatch() {
  struct Circle { radius: f64 }
  struct Square { side: f64 }

  trait Shape {
    fn area(&self) -> f64;
  }

  impl Shape for Square {
    fn area(&self) -> f64 {
      self.side * self.side
    }
  }

  impl Shape for Circle {
    fn area(&self) -> f64 {
      self.radius * self.radius * std::f64::consts::PI
    }
  }

  let shapes:[&Shape; 4] = [
    &Circle { radius: 1.0 },
    &Square { side: 3.0 },
    &Circle { radius: 2.0 },
    &Square { side: 4.0 }
  ];

  for (i, shape) in shapes.iter().enumerate() {
    println!("Shape #{} has area {}", i, shape.area());
  }
}
