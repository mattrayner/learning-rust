#![allow(dead_code)]
use std::mem;
mod stack_and_heap;
mod control_flow;
mod data_structures;

const MEANING_OF_LIFE:u8 = 42; // Constants have no fixed address

static mut Z:i32 = 123; // You can make

fn primitive_types() {
  // unsigned 0+
  let a:u8 = 123; // 8bits
  println!("a = {}", a);
//  a = 456; // Causes an error at compilation time as variables are immutable by default.

  // mut
  let mut b:i8 = 0;
  println!("b = {}", b);
  b = 42;
  println!("b = {}", b);

  let mut c = 123456789; //32-bit signed i32
  println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
  c = -123456789;
  println!("c = {}, after modification", c);

  // i8 u8 i16 u16 i32 u32 i64 u64
  let z:isize = 123; //isize/usize
  let size_of_z = mem::size_of_val(&z);

  println!("z = {}, takes up {} bytes, {}-bit os",
           z, size_of_z, size_of_z * 8);

  let d:char = 'x';
  println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

  let e = 2.5; // double-precision, 8 bytes, or 64 bits, f64 - can also be f32 for smaller memory usage
  println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

  // true false
  let f = false;
  println!("f = {}, size = {} bytes", f, mem::size_of_val(&f));

  let g = 4>0;
  println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
}

fn operators() {
  // arithmetic
  let mut a = 2+3*4; // +-*/
  println!("{}", a);
  a = a+1; // -- and ++ are not supported in Rust
  println!("{}", a);
  a -= 2; // a = a - 2
          // -= += /= %=

  println!("remainder of {} / {} = {}", a, 3, (a%3));

  let a_cubed = i32::pow(a, 3);
  println!("{} cubed = {}", a, a_cubed);

  let b = 2.5;
  let b_cubed = f64::powi(b, 3);
  let b_to_power_of_pi = f64::powf(b, std::f64::consts::PI);
  println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_power_of_pi);

  // bitwise
  let c = 1 | 2; // | OR & AND ^ XOR ! NOR
                 // 01 OR 10 == 11 == 3_10
  println!("1|2 = {}", c);
  let two_to_10 = 1 << 10;
  println!("2^10 = {}", two_to_10);

  // logical
  let pi_less_4 = std::f64::consts::PI < 4.0; // true
  println!("pi<4? {}", pi_less_4);
  // > <= >= ==
  let x = 5;
  let x_is_5 = x == 5; //true
  println!("{} = 5? {}", x, x_is_5);
}

fn scope_and_shadowing() {
//  let a = 123;
  let a = 1234; // You can re-declare a variable and the 'newest' value will be taken

  {
    let b = 156;
    println!("inside, b = {}", b);


    let a = 777;
    println!("inside, a = {}", a);
  }

  println!("outside, a = {}", a);
//  println!("outside, b = {}", b);
}

fn defining_and_using_constants() {
  /*
    At compilation time, references to constants get replaced with the values of those constants.

    At compilation time, the below will become:
    println!("{}", 42);
  */
  println!("{}", MEANING_OF_LIFE);

  /*
    You can use static variables, which have a fixed address in memory... They can also be mutable.

    However, when you are using or modifying a mutable static variable, all usages must be contained
    within an unsafe block.

    This effectively tells the compiler that you know what you are doing with this variable and
    promise to be careful.
  */
  unsafe {
    Z = 777;
    println!("{}", Z);
  }

  // You should use const, not static...
}

fn main() {
  println!("--- PRIMITIVE TYPES ---");
  primitive_types();

  println!("\n\r--- OPERATORS ---");
  operators();

  println!("\n\r--- SCOPE AND SHADOWING ---");
  scope_and_shadowing();

  println!("\n\r--- DEFINING AND USING CONSTANTS ---");
  defining_and_using_constants();

  println!("\n\r--- STACK AND HEAP ---");
  stack_and_heap::stack_and_heap();

  println!("\n\r--- IF STATEMENTS ---");
  control_flow::if_statement();

  println!("\n\r--- WHILE AND LOOP ---");
  control_flow::while_and_loop();

  println!("\n\r--- FOR LOOP ---");
  control_flow::for_loop();

  println!("\n\r--- MATCH STATEMENT ---");
  control_flow::match_statement();

  println!("\n\r--- DATA STRUCTURES ---");
  println!("\t--- Structures");
  data_structures::structures();

  println!("\t--- Enumerations");
  data_structures::enumerations();

  println!("\t--- Unions");
  data_structures::unions();

  println!("\t--- Options");
  data_structures::options();

  println!("\t--- Arrays");
  data_structures::arrays();
}
