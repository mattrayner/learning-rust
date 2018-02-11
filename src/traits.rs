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
