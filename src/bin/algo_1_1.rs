use std::io;

fn main() {
    let n = input_n();
    println!("{:?}", n/5+n%5);
}

fn input_n() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Failed to parse")
}

fn input() -> Vec<usize> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().split_whitespace()
        .map(|e| e.parse().expect("Failed to parse"))
        .collect()
}


