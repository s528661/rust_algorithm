use std::io;

fn main() {
    let mut n = input_n();
    let mut cnt = 0;
    
    loop {
        if n == 0 {
            break;
        }
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = n - 1;
        }
        cnt += 1;
    }
    println!("{:?}", cnt);
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


