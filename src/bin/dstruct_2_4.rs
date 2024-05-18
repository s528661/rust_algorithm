use std::io;

fn main() {
    let n = input_n();
    let mut a = input();
    let q = input_n();

    for _ in 0..q {
        let q_kv = input();
        if q_kv[0] == 0 {
            a.insert(q_kv[1], q_kv[2]);
        } else if q_kv[0] == 1 {
            a.remove(q_kv[1]);
        } else {
            println!("{:?}", a.iter().filter(|&&n| n == q_kv[1]).count());
        }
    }
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

