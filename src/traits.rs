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

// Dynamic dispatch
fn print(z:&Printable) {}

// Static dispatch
fn print_it<T: Printable>(z: T) {
  println!("{}", z.format);
}

/*
At compilation time, we get 'monomorphisation' -> the above print_it function gets compiled into
discreet method implementations for each type of Printable that uses it.

For example, the two uses below, actually create:
fn print_it(z: i32) {
  println!("{}", z.format);
}
fn print_it(z: String) {
  println!("{}", z.format);
}

These methods are then faster, and more efficient, to run as the calls to `print_it` below are
replaced with references to the specific print_it for that type.
*/

pub fn static_dispatch() {
  let a = 123;
  let b = "hello".to_string();

//  println!("{}", a.format());
//  println!("{}", b.format());

  println!("{}", print_it(a));
  println!("{}", print_it(b));
}