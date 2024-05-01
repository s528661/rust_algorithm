use std::io::stdin;

fn main(){
    let N: usize = input_n() as usize;
    let A = input();

    let mut cnt = 0;
    for i in 0..N-1 as usize {
        if A[i] < A[i+1] {
            cnt += 1;
        }
    }
    println!("{:?}", cnt);
}

fn input_n()->isize{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().parse().unwrap();
}

fn input()->Vec<usize>{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
}
