extern crate rand;
use self::rand::Rng;

pub fn consuming_crates() {
  let mut rng = rand::thread_rng();
  let b:bool = rng.gen();

  println!("Genreated random boolean: {}", b)
}