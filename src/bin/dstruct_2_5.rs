use std::io;

fn main() {
    let n = input_n();
    let mut A: Vec<_> = (1..(n+1)).collect();
    let mut index = 0;
    
    loop {
      if A.len() == 1 {
        break;
      }

      A.retain(|_| { index+=1; index % 2 == 0 });
    }
    println!("{:?}", A[0]);
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


