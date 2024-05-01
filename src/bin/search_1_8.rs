use std::io::stdin;

fn main(){
    let N = input_n();
    let A = input();

    println!("{:?}", A.iter().min().unwrap()); // If not used `unwrap` method, the output is `Some()`
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
