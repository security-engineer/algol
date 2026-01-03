use std::io;

fn main(){
    let mut buf = String::new();

    io::stdin().read_line(&mut buf).expect("Failed to Read line.");
    let val: i32 = buf.trim().parse::<i32>().unwrap();

    //let mut case_vec: Vec<i32> = Vec::new();

    //println!("{}", val);
    let mut result: i32 = 0;

    for x in 1..=val {
        
        // buf.clear();
        // io::stdin().read_line(&mut buf).expect("Failed to read line.");
        // let cal: Vec<i32> = buf.trim().split(' ').map(|s| s.parse::<i32>().unwrap()).collect();
        // case_vec.push(cal[0]+cal[1]);
    }

    for x in 0..case_vec.len() {
        println!("{}", case_vec[x]);
    }
}