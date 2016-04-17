extern crate rand;

use std::f64;
use rand::Rng;

fn main() {
  let e = f64::consts::E;
  println!("E: {}", e);

  let max = 1000;
  let pi: f64 = f64::consts::PI;
  let a: [f64; 4] = [
      rand::thread_rng().gen_range(0, max) as f64, 
      rand::thread_rng().gen_range(0, max) as f64, 
      rand::thread_rng().gen_range(0, max) as f64, 
      rand::thread_rng().gen_range(0, max) as f64,
  ];

  for &n in &a {
    let n2 = n/(max as f64);
    let nn = -1.0 * (n2/2.0);
    let tip = nn.powi(2);
    let top = e.powf(tip);
    let bottom = 2.0 * pi;
    let result = top.powf(tip)/bottom;

    println!("Gaussed {}: {:?}", n2, result);
  }
}
