use std::io;

fn main(){
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read line.");
    let mut count: i32 = buf.trim().parse::<i32>().unwrap(); buf.clear();

    let mut case_vec: Vec<(i32, i32)> = Vec::new();
    for _ in 1..=count {
        io::stdin().read_line(&mut buf).expect("Failed to read line.");
        let (x,y): (i32, i32) = buf.trim().split(' ').map(|s| s.parse::<i32>().unwrap()).collect(); buf.clear();
        case_vec.push((x,y));
    }
}