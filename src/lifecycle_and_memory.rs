#![allow(unused_must_use)]

use std::rc::Rc;
use std::thread;
use std::sync::{Mutex, Arc};

pub fn ownership() {
  let a = vec![1,2,3];
  let b = a;
//  println!("{:?}", a);
  println!("{:?}", b);
}

pub fn reference_counting() {
  struct Person {
    name: Rc<String>
  }

  impl Person {
    fn new(name: Rc<String>) -> Person {
      Person { name }
    }

    fn greet(&self) {
      println!("Hi, my name is {}", self.name)
    }
  }

  let name = Rc::new("Matt".to_string());
  println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
//  let matt = Person::new(name.clone()); // Create a duplicate and increment the reference counter
  {
    let matt = Person::new(name.clone()); // Create a duplicate and increment the reference counter
    println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
    matt.greet();
  }
//  matt.greet();
  println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
  println!("Name = {}", name);
}

pub fn automatic_reference_counting() {
  struct Person {
    name: Arc<String>
  }

  impl Person {
    fn new(name: Arc<String>) -> Person {
      Person { name }
    }

    fn greet(&self) {
      println!("Hi, my name is {}", self.name)
    }
  }

  let name = Arc::new("Matt".to_string());
  let person = Person::new(name.clone()); // Create a duplicate and increment the reference counter

  let t = thread::spawn(move || {
    person.greet();
  });
  println!("Name = {}", name);

  t.join();
}

pub fn mutex_for_thread_safe_mutability() {
  struct Person {
    name: Arc<String>,
    state: Arc<Mutex<String>>
  }

  impl Person {
    fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> Person {
      Person { name, state }
    }

    fn greet(&self) {
      let mut state = self.state.lock().unwrap();

      state.clear();
      state.push_str("Excited");

      println!("Hi, my name is {}, I am {}", self.name, state.as_str());
    }
  }

  let name = Arc::new("Matt".to_string());
  let state = Arc::new(Mutex::new("Bored".to_string()));
  let person = Person::new(name.clone(), state.clone()); // Create a duplicate and increment the reference counter

  let t = thread::spawn(move || {
    person.greet();
  });

  println!("Name = {}, state = {}", name, state.lock().unwrap().as_str());

  t.join();

}