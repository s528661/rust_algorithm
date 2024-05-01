use td::io::stdin;
use std::collections::HashMap;

fn main(){
    let k = input_n();
    let v = vec![3,1,4,1,5,9,2,6,5,3];

    println!("{:?}", v[k]);
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
}s
