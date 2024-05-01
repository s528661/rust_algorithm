use std::io::stdin;

fn main(){
    let N = input_n();
    let A = input();

    let mut max_A = 0;
    let mut max_i: i32 = -1 as i32;
    for (i, &a) in A.iter().enumerate() {
        if max_A < a {
            max_A = a;
            max_i = i as i32;
        }
    }
    println!("{:?}", max_i);
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
