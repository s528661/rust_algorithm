use std::io;

fn main() {
    let mut n = input_n();
    let mut A = input();
    let mut add = 0;
    let mut cnt = 0;

    for (k, v) in vec![50, 10, 5, 1].iter().zip(A.iter()) {
        add = if n/k > *v { *v } else { n/k };
        cnt += &add;
        n = n - &add * k;
    }

    println!("{:?}", &cnt);
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


