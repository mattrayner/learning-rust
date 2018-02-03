use std::mem;

fn main() {
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
