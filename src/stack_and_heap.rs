#![allow(dead_code)]
use std::mem;

struct Point
{
  x: f64,
  y: f64
}

fn origin() -> Point
{
  Point{x: 0.0, y: 0.0}
}

pub fn stack_and_heap(){
  /*
      --- STACK ---
      The stack is used for temporary storage.. As you add variables, call functions etc, all of the
      assignments are added to the stack. Then, when those variables are no longer needed, they can be
      'popped'.

      This means we do not need to manually `dealloc` memory, it happens automatically. However, the
      main limitation of the stack is that is is limited.
    */
  let x = 5; //i32
  let y = 32; //i32
  // function(x, y) (later calls) => g(z)
  // x, y, and now z, are all added to the stack.
  println!("x, and y's values on the stack, are: {} {}", x, y);

  /*
    --- HEAP ---
    The heap is used for more long-term storage of data. When you specifically add something to the
    heap, and assign that to a variable, the variable's value in the stack is simply a pointer to a
    memory location in the heap.
  */
  let a = Box::new(5);
  /*
    With the stack examples above, x's value on the stack would be the i32, 5. In the above line,
    a's value in the stack would be a memory address in the heap. That memory address in the heap,
    which then contain the i32 5.

    This extra step of getting to the value, 5, stored in the heap, has to be accounted for in our
    code by using the de-reference operator '*'.
  */
  println!("the value of a, stored on the heap, is: {}", *a);

  let p1 = origin();
  let p2 = Box::new(origin()); // Allocate the Point onto the heap

  println!("p1 takes up {} bytes", mem::size_of_val(&p1));
  println!("p2 takes up {} bytes", mem::size_of_val(&p2));

  let p3 = *p2; // De-reference p2, making a copy of the Point back onto the stack
  println!("the x value of p3 is {}", p3.x);
}