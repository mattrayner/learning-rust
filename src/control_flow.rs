pub fn if_statement() {
  let temp = 35;

  if temp > 30 {
    println!("really hot outside");
  } else if temp < 10 {
    println!("really, cold!");
  } else {
    println!("temprature is okay");
  }

  let day = if temp > 20 { "sunny" } else { "cloudy" };
  println!("today is {}", day);

  println!("it is {}",
           if temp > 20 { "hot" } else if temp < 10 { "cold" } else { "ok" });

  println!("is is {}",
           if temp > 20 {
             if temp > 30 { "very hot" } else { "hot" }
           } else if temp < 10 { "cold" } else { "ok" });
}

pub fn while_and_loop() {
  let mut x = 1;

  while x < 1000 {
    x *=2;

    if x == 64 { continue; } // Move to the next iteration. Equivalent of `next`.

    println!("x = {}", x);
  }

  let mut y = 1;
  loop { // while true
    y *= 2;
    println!("y = {}", y);

    if y == 1<<10 { break; } // Kick us out of this loop
  }
}