use std::io::stdin;
use std::collections::HashMap;

fn main(){
    let Q = input_n();
    let mut v = vec![3,1,4,1,5,9,2,6,5,3];

    for q in 0..Q {
        let binkv = input();
        
        if binkv[0] == 0 {
            println!("{:?}", v[binkv[1]]);
        } else {
            v[binkv[1]] = binkv[2];
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
