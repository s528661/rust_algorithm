use std::io::stdin;

fn main(){
    let NV = input();
    let mut A = input();

    A.reverse();

    for (i, a) in A.iter().enumerate() {
        if a == &NV[1] {
            println!("{:?}", NV[0]-i-1);
            return;
        }
    }

    println!("{:?}", -1);
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
