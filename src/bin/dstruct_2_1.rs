use std::io::stdin;
use std::collections::HashMap;

fn main(){
  let Nk = input();
  let A = input();

  println!("{:?}\n{:?}",A[Nk[1]],A[Nk[0]-Nk[1]-1]);
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
