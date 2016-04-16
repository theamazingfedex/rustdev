use std::f64;

fn main() {
  let e = f64::consts::E;
  let pi: f64 = f64::consts::PI;
  //let e = 1 as f64;
  let a: [f64; 4] = [0.0, 0.2, 0.5, 1.0];
  //let a = [1, 9, 21];
  for &n in &a {
    let nn: f64 = -1.0 * (n/2.0);
    let tip: f64 = nn.powi(2);
    let top: f64 = e.powf(tip);
    let bottom: f64 = 2.0 * pi;
    let result: f64 = top.powf(tip)/bottom;

    println!("Gaussed {}: {:?}", n, result);
  }
  println!("E: {}", e);
}
