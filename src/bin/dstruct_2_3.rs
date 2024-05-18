use std::io::stdin;
use std::collections::HashMap;

fn main(){
  let N = input_n();
  let mut A = input();
  let Q = input_n();

  for _q in 0..Q {
    let q_y = input();
    if q_y[0] == 0 {
        A.push(q_y[1]);
    } else {
      if A.len() == 0 {
        println!("Error");
      } else {
        println!("{:?}", A.pop().unwrap());
      }
    }
  }


}

fn input_n()->usize{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().parse().unwrap();
}

fn input()->Vec<usize>{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
}
