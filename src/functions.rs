fn print_value(x: i32) {
  println!("value = {}", x);
}

fn increate(x: &mut i32) {
  *x += 1;
}

fn product(x: i32, y: i32) -> i32 {
  x * y
}

pub fn functions_and_function_arguments() {
  print_value(42);

  let mut z = 1;
  increate(&mut z);
  println!("z increased to: {}", z);

  let a = 5;
  let b = 4;
  let p = product(a, b);
  println!("{} * {} = {}", a, b, p);
}

pub fn methods() {
  struct Point {
    x: f64,
    y: f64
  }

  struct Line {
    start: Point,
    end: Point
  }

  impl Line {
    fn len(&self) -> f64 {
      let dx = self.start.x - self.end.x;
      let dy = self.start.y - self.end.y;

      (dx*dx+dy*dy).sqrt()
    }
  }

  let p = Point { x: 3.0, y: 4.0 };
  let p2 = Point { x: 5.0, y: 10.0 };
  let my_line = Line { start: p, end: p2 };
  println!("Line length = {}", my_line.len());
}

fn say_hello() {
  println!("Hello, world!");
}

pub fn closures() {
  let sh = say_hello;
  sh();

  // In-line function that does not exist ourside of the closure's function scope.
  let plus_one = |x: i32| -> i32 { x + 1 };

  let x = 1;
  println!("{} + 1 = {}", x, plus_one(x));

  let mut two = 2;
  {
    let plus_two = |x|
      {
        let mut z = x;
        z += two;
        z
      };
    println!("{} + 2 = {}", 7, plus_two(7));

    /*
    If you use 'two' inside a closure, you can't borrow a mutable version of it.

    You can get around this by wrapping your closure in a scope so that it's borrowing of two is
    deleted before we borrow it.
    */
    //  let borrow_two = &mut two;
  }
  let borrow_two = &mut two;
  println!("borrow_two = {}", borrow_two);

  // T: by value
  // T& by reference
  // &mut T by mutable
  let plus_three = |x: &mut i32| { *x += 3 };

  let mut f = 12;
  plus_three(&mut f);
  println!("12 + 3 = {}", f);
}

fn is_even(x: i32) -> bool {
  x % 2 == 0
}

pub fn higher_order_functions() {
  let limit = 500;
  let mut sum = 0;

  for i in 0.. {
    let isq = i * i;

    if isq > limit { break; }
    else if is_even(isq) { sum += isq; }
  }

  println!("loop sum = {}", sum);

  let sum2 =
    (0..)
      .map(|x| x*x)
      .take_while(|&x| x <= limit)
      .filter(|x| is_even(*x))
      .fold(0, |sum, x| sum + x);
  println!("higher order function sum = {}", sum2);
}
