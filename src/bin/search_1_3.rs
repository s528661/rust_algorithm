use std::io::stdin;

fn main(){
    let N = input_n();
    let A = input();

    let mut cnt = 0;
    for a in A {
        if a > 0 {
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

fn input()->Vec<isize>{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
}
