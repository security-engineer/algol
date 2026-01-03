use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read line.");
    let mut total: i64 = buf.trim().parse::<i64>().unwrap(); buf.clear();
    io::stdin().read_line(&mut buf).expect("Failed to read line.");
    let mut count: i32 = buf.trim().parse::<i32>().unwrap(); buf.clear();
    
    
    let mut result: i64 = 0;
    for x in 1..=count {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("Failed to Read line.");
        let mut temp: Vec<i32> = buf.trim().split(' ').map(|s| s.parse::<i32>().unwrap()).collect();
        result += (temp[0]*temp[1]) as i64;
    }

    if total == result {
        println!("Yes");
    } else {
        println!("No");
    }

    
}