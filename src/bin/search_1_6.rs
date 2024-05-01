use std::io::stdin;

fn main(){
    let N = input_n();
    let A = input();

    let max_A = A.iter().max().unwrap();

    println!("{:?}", max_A);
}

fn input_n()->isize{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().parse().unwrap();
}

fn input()->Vec<isize>{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
}
