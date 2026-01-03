use std::io;
use std::collections::HashMap;


fn main() {

    let mut buf = String::new();

    io::stdin().read_line(&mut buf).expect("Failed to read line.");

    let mut buf_vec: Vec<i32> = buf.trim().split(' ').map(|s| s.parse::<i32>().unwrap()).collect();
    let mut count_map: HashMap<i32, i32> = HashMap::new();

    for x in buf_vec {
        
        let count = count_map.entry(x).or_insert(0);
        *count += 1;
    }
    

    let max_entry = count_map.iter().max_by_key(|entry| entry.1);
    let max_val = max_entry.unwrap();

    if *max_val.1 == 3 {
        println!("{}", 10000+(*max_val.0)*1000)
    } else if *max_val.1 == 2 {
        println!("{}", 1000+(*max_val.0*100))
    } else {
        println!("{}", count_map.iter().max_by_key(|entry| entry.0).unwrap().0*100)
    }
}
