use std::io;

fn main(){
    let mut buf = String::new();

    io::stdin().read_line(&mut buf).expect("Failed to read line.");
    let mut count: i32 = buf.trim().parse::<i32>().unwrap(); buf.clear();

    for x in 1..=(count/4) {
        print!("long ",);
    }
    println!("int");
}