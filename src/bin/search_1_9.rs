use std::io::stdin;
use std::collections::HashMap;

fn main(){
    let N = input_n();
    let mut A = input();

    A.sort();

    let mut hist: HashMap<usize, i32> = HashMap::new();
    for i in 1..10 {
        hist.insert(i, 0);
    }

    for a in &A {
        match hist.get(&a) {
            Some(cnt) => { hist.insert(*a, cnt+1); }
            None => {}
        }
    }

    for i in 1..10 {
        println!("{:?}", hist.get(&i).unwrap());
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
