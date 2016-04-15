use std::f64;

fn main() {
  //let e = f64::consts::E;
  let e = 1 as f64;
  //let a: [i32; 4] = [0, 1, 2, 4];
  let a = [1, 9, 21];
  for &n in &a {
    let nn: i32 = (n as i32 * (1/2)) * -1;
    let tip: f64 = ((nn) as f64).powi(2);
    let top: f64 = e.powf(tip);
    let middle: f64 = f64::consts::PI;
    let bottom: f64 = 2 as f64 * (middle);
    let result: f64 = top.powf(tip)/bottom;

    println!("Gaussed {}: {:?}", n, result);
  }
  println!("E: {}", e);
}
