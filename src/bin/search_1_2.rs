use std::io::stdin;

fn main(){
    let NV = input();
    let A = input();

    let mut cnt = 0;
    for a in A {
        if NV[1] == a {
          cnt += 1;
        }
    }

    println!("{:?}", cnt);
}

fn input()->Vec<usize>{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
}
